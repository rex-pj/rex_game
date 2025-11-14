import { redirect, type Actions } from "@sveltejs/kit";
import { UserApi } from "../../../../lib/api/userApi";
import { fail } from "@sveltejs/kit";
import type { ResetPasswordRequest } from "$lib/models/user";
import { UserServerApiOptions } from "$lib/api/apiOptions";

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const userApi = new UserApi(new UserServerApiOptions(cookies));
    const formData = await request.formData();

    const dataObject: { [key: string]: any } = {};
    for (const [key, value] of formData.entries()) {
      dataObject[key] = value;
    }

    const data = dataObject as ResetPasswordRequest;
    const response = await userApi.resetPassword(fetch, data);
    if (!response || response.error || response.field_errors) {
      const token = data.token;
      return fail(400, {
        message: response?.error || "Request to reset password failed",
        field_errors: response?.field_errors,
        values: { token },
      });
    }

    throw redirect(303, `/account/login`);
  },
};
