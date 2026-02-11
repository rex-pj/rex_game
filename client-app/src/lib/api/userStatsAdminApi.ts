import type { AdminUserStats } from "$lib/models/admin-user-stats";
import { BaseApi } from "./baseApi";

export class UserStatsAdminApi extends BaseApi {
  private baseUrl = "/admin/user-stats";

  async getList(fetch: Function, page: number, page_size: number) {
    const response: Response = await this.get(
      fetch,
      this.baseUrl,
      new URLSearchParams({ page: page.toString(), page_size: page_size.toString() }),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch user stats");
    }
    return (await response.json()) as { items: AdminUserStats[]; total_count: number };
  }

  async resetStats(fetch: Function, userId: number): Promise<boolean> {
    const response: Response = await this.put(
      fetch,
      `${this.baseUrl}/${userId}/reset`,
      {},
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to reset user stats");
    }
    return (await response.json()) as boolean;
  }
}
