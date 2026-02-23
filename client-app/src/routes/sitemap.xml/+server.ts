import type { RequestHandler } from '@sveltejs/kit';

const PUBLIC_ROUTES = [
  { path: '/', priority: '1.0', changefreq: 'daily' },
  { path: '/flashcard', priority: '0.9', changefreq: 'daily' },
  { path: '/leaderboard', priority: '0.8', changefreq: 'hourly' },
  { path: '/achievements', priority: '0.7', changefreq: 'weekly' },
  { path: '/account/login', priority: '0.5', changefreq: 'monthly' },
  { path: '/account/signup', priority: '0.6', changefreq: 'monthly' },
  { path: '/account/forgot-password', priority: '0.3', changefreq: 'monthly' },
];

export const GET: RequestHandler = ({ url }) => {
  const base = url.origin;
  const today = new Date().toISOString().split('T')[0];

  const urls = PUBLIC_ROUTES.map(
    (route) => `
  <url>
    <loc>${base}${route.path}</loc>
    <lastmod>${today}</lastmod>
    <changefreq>${route.changefreq}</changefreq>
    <priority>${route.priority}</priority>
  </url>`
  ).join('');

  const sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>`;

  return new Response(sitemap, {
    headers: {
      'Content-Type': 'application/xml',
      'Cache-Control': 'max-age=3600',
    },
  });
};
