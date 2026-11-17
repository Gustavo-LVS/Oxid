// Interface provisória do Oxid. Fala com o lado Rust via comandos do Tauri.
const invoke = window.__TAURI__.core.invoke;
const $ = (id) => document.getElementById(id);

let registering = false;
let currentServer = null;

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
    if (c.kind === "text") text.append(li("# " + c.name, () => setStatus(`Canal #${c.name}: o chat de texto chega no passo 4.`)));
    else voice.append(li("🔊 " + c.name, () => setStatus(`Canal de voz ${c.name}: a voz entra no app depois da prova de conceito.`)));
  }
  setStatus("");
}

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
