import type { Achievement } from "$lib/models/achievement";
import { BaseApi } from "./baseApi";

export class AchievementAdminApi extends BaseApi {
  private baseUrl = "/admin/achievements";

  async getList(
    fetch: Function,
    page: number,
    page_size: number,
    name?: string | null
  ) {
    const params: Record<string, string> = {
      page: page.toString(),
      page_size: page_size.toString(),
    };
    if (name && name.trim() !== "") {
      params.name = name;
    }
    const response: Response = await this.get(
      fetch,
      this.baseUrl,
      new URLSearchParams(params),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch achievements");
    }
    return (await response.json()) as { items: Achievement[]; total_count: number };
  }

  async getById(fetch: Function, id: number) {
    const response: Response = await this.get(
      fetch,
      `${this.baseUrl}/${id}`,
      new URLSearchParams(),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch achievement");
    }
    return (await response.json()) as Achievement;
  }

  async create(
    fetch: Function,
    data: { code: string; name: string; description?: string; icon?: string; points: number; category?: string }
  ) {
    const response: Response = await this.post(fetch, this.baseUrl, data, {
      observe: true,
    });
    if (response.status !== 200) {
      const errorText = await response.text();
      throw new Error(errorText || "Failed to create achievement");
    }
    return (await response.json()) as number;
  }

  async update(
    fetch: Function,
    id: number,
    data: { code?: string; name?: string; description?: string; icon?: string; points?: number; category?: string }
  ) {
    const response: Response = await this.patch(fetch, `${this.baseUrl}/${id}`, data, {
      observe: true,
    });
    if (response.status !== 200) {
      const errorText = await response.text();
      throw new Error(errorText || "Failed to update achievement");
    }
    return (await response.json()) as boolean;
  }

  async deleteById(fetch: Function, id: number) {
    const response: Response = await this.delete(fetch, `${this.baseUrl}/${id}`, {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to delete achievement");
    }
    return (await response.json()) as number;
  }

  async toggleActive(fetch: Function, id: number): Promise<boolean> {
    const response: Response = await this.put(
      fetch,
      `${this.baseUrl}/${id}/toggle-active`,
      {},
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to toggle achievement active status");
    }
    return (await response.json()) as boolean;
  }
}
