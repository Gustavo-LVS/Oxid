// Interface provisória do Oxid. Fala com o lado Rust via comandos do Tauri.
const invoke = window.__TAURI__.core.invoke;
const $ = (id) => document.getElementById(id);

let registering = false;
let currentServer = null;
let currentChannel = null;

function setStatus(text) { $("status").textContent = text; }

function li(text, onClick, active) {
  const el = document.createElement("li");
  el.textContent = text;
  if (active) el.classList.add("active");
  if (onClick) el.addEventListener("click", onClick);
  return el;
}

// ---- Login / cadastro ----
function setRegistering(on) {
  registering = on;
  document.querySelectorAll(".register-only").forEach((el) => (el.hidden = !registering));
  $("auth-form").email.required = registering;
  $("auth-submit").textContent = registering ? "Criar conta" : "Entrar";
  $("auth-toggle").textContent = registering ? "Já tenho conta" : "Criar conta";
}
$("auth-toggle").addEventListener("click", () => setRegistering(!registering));

$("auth-form").addEventListener("submit", async (ev) => {
  ev.preventDefault();
  const f = ev.target;
  $("auth-error").textContent = "";
  try {
    const res = registering
      ? await invoke("register", { apiUrl: f.api_url.value, username: f.login.value, email: f.email.value, password: f.password.value })
      : await invoke("login", { apiUrl: f.api_url.value, login: f.login.value, password: f.password.value });
    f.password.value = "";
    $("me").textContent = res.user.display_name || res.user.username;
    $("auth").hidden = true;
    $("app").hidden = false;
    await loadServers();
  } catch (e) {
    $("auth-error").textContent = e;
  }
});

$("logout").addEventListener("click", async () => {
  await invoke("logout");
  currentServer = null;
  closeChannel();
  setRegistering(false);
  $("app").hidden = true;
  $("auth").hidden = false;
});

// ---- Servidores ----
async function loadServers(selectId) {
  const servers = await invoke("list_servers");
  const list = $("server-list");
  list.replaceChildren();
  for (const s of servers) {
    const b = document.createElement("button");
    b.textContent = s.name.slice(0, 2).toUpperCase();
    b.title = s.name;
    if (s.id === (selectId || currentServer?.server.id)) b.classList.add("active");
    b.addEventListener("click", () => openServer(s.id));
    const item = document.createElement("li");
    item.append(b);
    list.append(item);
  }
  const target = selectId || currentServer?.server.id || servers[0]?.id;
  if (target) await openServer(target);
  else setStatus("Você ainda não está em nenhum servidor. Crie um com + ou entre por convite.");
}

async function openServer(id) {
  try {
    currentServer = await invoke("server_detail", { serverId: id });
  } catch (e) {
    return setStatus(e);
  }
  document.querySelectorAll("#server-list button").forEach((b) => b.classList.toggle("active", b.title === currentServer.server.name));
  $("server-name").textContent = currentServer.server.name;
  $("invite-btn").hidden = false;
  $("invite-code").hidden = true;
  const text = $("text-channels"), voice = $("voice-channels");
  text.replaceChildren();
  voice.replaceChildren();
  for (const c of currentServer.channels) {
    if (c.kind === "text") text.append(li("# " + c.name, () => openChannel(c)));
    else voice.append(li("🔊 " + c.name, () => setStatus(`Canal de voz ${c.name}: a voz entra no app depois da prova de conceito.`)));
  }
  setStatus("");
  const firstText = currentServer.channels.find((c) => c.kind === "text");
  if (firstText) await openChannel(firstText);
  else closeChannel();
}

// ---- Chat de texto ----
function closeChannel() {
  currentChannel = null;
  $("chat").hidden = true;
}

function renderMessage(m) {
  const el = document.createElement("li");
  const time = document.createElement("time");
  time.dateTime = m.created_at;
  time.textContent = new Date(m.created_at).toLocaleTimeString("pt-BR", { hour: "2-digit", minute: "2-digit" });
  const author = document.createElement("span");
  author.className = "author";
  author.textContent = m.author.display_name || m.author.username;
  const body = document.createElement("span");
  body.textContent = m.content;
  el.dataset.id = m.id;
  el.append(time, author, body);
  return el;
}

function appendMessage(m) {
  const list = $("messages");
  if (list.querySelector(`[data-id="${m.id}"]`)) return; // já veio pelo envio ou pelo gateway
  list.append(renderMessage(m));
  list.scrollTop = list.scrollHeight;
}

async function openChannel(c) {
  currentChannel = c;
  document.querySelectorAll("#text-channels li").forEach((el) => el.classList.toggle("active", el.textContent === "# " + c.name));
  $("chat-title").textContent = "# " + c.name;
  $("messages").replaceChildren();
  $("chat").hidden = false;
  try {
    const history = await invoke("list_messages", { channelId: c.id, before: null });
    if (currentChannel?.id !== c.id) return;
    history.forEach(appendMessage);
  } catch (e) {
    setStatus(e);
  }
}

$("message-form").addEventListener("submit", async (ev) => {
  ev.preventDefault();
  const input = ev.target.content;
  const content = input.value.trim();
  if (!content || !currentChannel) return;
  input.value = "";
  try {
    appendMessage(await invoke("send_message", { channelId: currentChannel.id, content }));
  } catch (e) {
    input.value = content;
    setStatus(e);
  }
});

window.__TAURI__.event.listen("gateway", ({ payload }) => {
  if (payload.op === "message_create") {
    const m = payload.d.message;
    if (m.channel_id === currentChannel?.id) appendMessage(m);
  }
});
window.__TAURI__.event.listen("gateway-closed", () => setStatus("Conexão em tempo real caiu. Saia e entre de novo para reconectar."));

$("new-server").addEventListener("click", () => $("server-dialog").showModal());
$("server-dialog").addEventListener("close", async () => {
  const dlg = $("server-dialog");
  const name = $("server-form").name.value.trim();
  $("server-form").reset();
  if (dlg.returnValue !== "ok" || !name) return;
  try {
    const created = await invoke("create_server", { name });
    await loadServers(created.server.id);
  } catch (e) {
    setStatus(e);
  }
});

$("invite-btn").addEventListener("click", async () => {
  try {
    const inv = await invoke("create_invite", { serverId: currentServer.server.id });
    $("invite-code").textContent = inv.code;
    $("invite-code").hidden = false;
  } catch (e) {
    setStatus(e);
  }
});

$("join-form").addEventListener("submit", async (ev) => {
  ev.preventDefault();
  try {
    const s = await invoke("accept_invite", { code: ev.target.code.value });
    ev.target.reset();
    await loadServers(s.id);
    setStatus(`Você entrou em ${s.name}.`);
  } catch (e) {
    setStatus(e);
  }
});

// ---- Push-to-talk (atalho global registrado no lado Rust) ----
invoke("ptt_shortcut_label").then((k) => ($("ptt-key").textContent = k));
window.__TAURI__.event.listen("ptt", ({ payload: pressed }) => {
  $("ptt").classList.toggle("on", pressed);
  $("ptt").firstChild.textContent = pressed ? "Falando: " : "Push-to-talk: ";
});

// ---- Atualização automática ----
function showUpdate(u) {
  if (!u) return;
  $("update-text").textContent = `Versão ${u.version} baixada.`;
  $("update-bar").hidden = false;
}
window.__TAURI__.event.listen("update-ready", ({ payload }) => showUpdate(payload));
invoke("pending_update").then(showUpdate);
$("update-restart").addEventListener("click", () => invoke("restart_to_update"));
