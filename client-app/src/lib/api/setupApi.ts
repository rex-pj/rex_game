import type { BaseApiOptions } from "./apiOptions";
import { BaseApi } from "./baseApi";

export class SetupApi extends BaseApi {
  private readonly baseUrl = "/setup";
  constructor(options: BaseApiOptions) {
    super(options);
  }

  async setup(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: {
      admin_email: string;
      admin_username: string;
      admin_password: string;
      admin_first_name?: string;
      admin_last_name?: string;
    }
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to setup system");
    }
    return await response.json();
  }
}
