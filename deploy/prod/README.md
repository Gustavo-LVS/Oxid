# Servidor de produção (você + amigos)

Uma VM Linux com Docker roda tudo: API, Postgres, LiveKit (voz) e Caddy (HTTPS).
Requisitos mínimos: 1 vCPU, 1 GB de RAM (2 GB recomendado), IP público.

## 1. Portas no firewall da nuvem

| Porta | Protocolo | Para quê |
|-------|-----------|----------|
| 80, 443 | TCP | HTTPS (API e sinalização da voz) |
| 7881 | TCP | Voz por TCP (redes que bloqueiam UDP) |
| 50000-50100 | UDP | Voz (caminho normal) |

Na Oracle Cloud é preciso liberar nos dois lugares: na *Security List* da VCN e
no firewall da própria VM (as imagens Ubuntu vêm com iptables fechado):

```sh
sudo iptables -I INPUT -p tcp -m multiport --dports 80,443,7881 -j ACCEPT
sudo iptables -I INPUT -p udp --dport 50000:50100 -j ACCEPT
sudo netfilter-persistent save
```

## 2. Instalar e subir

```sh
curl -fsSL https://get.docker.com | sudo sh
git clone https://github.com/Gustavo-LVS/Oxid.git && cd Oxid/deploy/prod
cp .env.example .env
# Preencha o .env: OXID_DOMAIN e um segredo novo em cada campo vazio.
for k in POSTGRES_PASSWORD OXID_JWT_SECRET LIVEKIT_API_SECRET; do
  sed -i "s/^$k=$/$k=$(openssl rand -hex 32)/" .env
done
sudo docker compose up -d --build
```

Sem domínio próprio, use `OXID_DOMAIN=<ip-com-hifens>.sslip.io` (ex.: IP
`203.0.113.5` vira `203-0-113-5.sslip.io`): o Caddy tira o certificado sozinho.

Teste: `curl https://SEU_DOMINIO/health` deve responder `ok`.

## 3. No app

Em "Servidor da API", coloque `https://SEU_DOMINIO`. Cada amigo cria a conta
no próprio app; você cria um servidor, clica em **Convidar** e manda o código.

## Atualizar o servidor

```sh
cd Oxid && git pull && cd deploy/prod && sudo docker compose up -d --build
```

## Backup do banco

```sh
sudo docker compose exec postgres pg_dump -U oxid oxid > oxid-$(date +%F).sql
```
