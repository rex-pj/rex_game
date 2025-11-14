import { ACCESS_TOKEN } from "$lib/common/contants";
import type { Cookies } from "@sveltejs/kit";
import type JsCookies from "js-cookie";

export interface BaseApiOptions {
  readonly tokenKey: string;
  cookies: Cookies | typeof JsCookies;
}

export class AdminServerApiOptions implements BaseApiOptions {
  constructor(cookies: Cookies) {
    this.cookies = cookies;
    this.tokenKey = ACCESS_TOKEN.ADMIN_ACCESS_TOKEN;
  }
  cookies: Cookies;
  readonly tokenKey: string;
}

export class AdminClientApiOptions implements BaseApiOptions {
  constructor(cookies: typeof JsCookies) {
    this.cookies = cookies;
    this.tokenKey = ACCESS_TOKEN.ADMIN_ACCESS_TOKEN;
  }
  cookies: typeof JsCookies;
  readonly tokenKey: string;
}

export class UserServerApiOptions implements BaseApiOptions {
  constructor(cookies: Cookies) {
    this.cookies = cookies;
    this.tokenKey = ACCESS_TOKEN.USER_ACCESS_TOKEN;
  }
  cookies: Cookies;
  readonly tokenKey: string;
}

export class UserClientApiOptions implements BaseApiOptions {
  constructor(cookies: typeof JsCookies) {
    this.cookies = cookies;
    this.tokenKey = ACCESS_TOKEN.USER_ACCESS_TOKEN;
  }
  cookies: typeof JsCookies;
  readonly tokenKey: string;
}
