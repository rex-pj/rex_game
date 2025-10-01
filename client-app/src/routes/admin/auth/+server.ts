import { AuthenticateApi } from "$lib/api/authenticateApi";
import { json, type RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ cookies, fetch }) => {
  const authenticateApi = new AuthenticateApi(cookies);
  const refresh_token = cookies.get("refresh_token");

  if (!refresh_token) {
    return new Response(JSON.stringify({ message: "No refresh token found" }), { status: 400 });
  }

  let response: Response;
  try {
    response = await authenticateApi.refreshToken(fetch);
  } catch (error) {
    console.error("Error during token refresh:", error);
    return new Response(JSON.stringify({ message: "Network error" }), { status: 500 });
  }

  if (!response.ok) {
    let msg: string;
    try {
      msg = await response.text();
    } catch {
      msg = "Unknown error";
    }
    console.error("Token refresh failed:", response.status, msg);
    return new Response(JSON.stringify({ message: "Token refresh failed" }), { status: 400 });
  }

  const loginResult = await response.json();
  authenticateApi.setAccessToken(loginResult, cookies);
  authenticateApi.setRefreshToken(response, cookies);

  return json({ success: true });
};
