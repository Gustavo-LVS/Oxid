# Site de download do Oxid

Astro estático: detecta o sistema do visitante e aponta para o instalador do último release no GitHub. Páginas: início, novidades, status, termos e privacidade (rascunhos para revisão jurídica).

```bash
npm install
npm run dev     # http://localhost:4321
npm run build   # gera dist/ para Cloudflare Pages ou Vercel
```

Antes de publicar, ajuste `src/config.ts` (repositório e URL da API) e `site` em `astro.config.mjs`.
