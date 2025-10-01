import { type Actions } from "@sveltejs/kit";
import { UserApi } from "../../../../lib/api/userApi";
import { fail } from "@sveltejs/kit";

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const userApi = new UserApi(cookies);
    const { token } = await request.json();
    if (!token) {
      return fail(400, {
        message: "Invalid or missing token.",
      });
    }

    const response = await userApi.confirm(fetch, { token });
    return response;
  },
};
