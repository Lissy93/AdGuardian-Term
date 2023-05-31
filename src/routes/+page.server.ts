
/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const readmeUrl = 'https://raw.githubusercontent.com/Lissy93/AdGuardian-Term/main/.github/README.md';
  const res = await fetch(readmeUrl);
  const markdown = await res.text();

  return { readme: markdown };
}
