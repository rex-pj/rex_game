export const isLinkValid = (input: string): { valid: boolean; href?: string; error?: string } => {
  const rawUrl = input.trim();
  if (!rawUrl) {
    return { valid: false, error: "URL is empty" };
  }

  const invalidProtocols = [
    "javascript:",
    "data:",
    "file:",
    "vbscript:",
    "ws:",
    "wss:",
    "view-source:",
    "ftp:",
    "sftp:",
    "ftps:",
    "ssh:",
    "tel:",
  ];
  if (!rawUrl || invalidProtocols.some((proto) => rawUrl.toLowerCase().startsWith(proto))) {
    return { valid: false, error: "URL is not valid" };
  }

  // Validate email
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (emailRegex.test(rawUrl)) {
    return { valid: true, href: `mailto:${rawUrl}` };
  }

  // Allow mailto: links
  if (/^mailto:/i.test(rawUrl)) {
    const emailPart = rawUrl.replace(/^mailto:/i, "");
    if (emailRegex.test(emailPart)) {
      return { valid: true, href: rawUrl };
    }
    return { valid: false, error: "URL is not valid" };
  }

  // Restrict to http/https URLs
  let urlStr = rawUrl;
  if (!/^https?:\/\//i.test(urlStr)) {
    urlStr = `https://${urlStr}`; // auto-add https if no scheme
  }

  try {
    const url = new URL(urlStr);
    if (!["http:", "https:"].includes(url.protocol)) {
      return { valid: false, error: "Only accept http/https" };
    }
    return { valid: true, href: url.toString() };
  } catch {
    return { valid: false, error: "URL is not valid" };
  }
};
