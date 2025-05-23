import type { CookieSerializeOptions } from "cookie";

class CookieHelper {
  parseSetCookie(setCookies: string[], key: string) {
    if (!setCookies) {
      return null;
    }

    const setCookie = setCookies.find((cookie: string) => cookie.includes(key)) as string;
    if (!setCookie) {
      console.error("Invalid setCookie:");
      return null;
    }

    const cookieParts = setCookie.split(";").map((cookie: string) => cookie.trim());
    const [nameValue, ...attrs] = cookieParts;
    const [, value] = nameValue.split("=");
    const options = this.getCookieOptions(attrs);
    return {
      value,
      options,
    };
  }

  private getCookieOptions(attrs: string[]) {
    const options: CookieSerializeOptions & { path: string } = {
      httpOnly: false,
      secure: false,
      sameSite: "lax",
      path: "/",
    };

    for (const attr of attrs) {
      const [k, v] = attr.split("=");
      switch (k.toLowerCase()) {
        case "httponly":
          options.httpOnly = true;
          break;
        case "secure":
          options.secure = true;
          break;
        case "samesite":
          options.sameSite = v as true | false | "lax" | "strict" | "none" | undefined;
          break;
        case "path":
          options.path = v;
          break;
        case "domain":
          options.domain = v;
          break;
        case "max-age":
          options.maxAge = parseInt(v);
          break;
        case "expires":
          options.expires = new Date(v);
          break;
      }
    }

    return options;
  }
}

export { CookieHelper };
