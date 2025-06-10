import { jwtDecode } from "jwt-decode";
import Cookies from "js-cookie";
import { goto } from "$app/navigation";

export const logout = async () => {
  fetch("/authentication", {
    method: "DELETE",
  }).then((rs) => {
    Cookies.remove("access_token");
    Cookies.remove("access_token_exp");
    goto("/admin/authentication");
  });
};

export const isTokenExpired = (token: string) => {
  try {
    const decoded = jwtDecode(token);
    if (!decoded || !decoded.exp) {
      throw new Error("Invalid token");
    }
    const currentTime = Math.floor(Date.now() / 1000);
    return decoded.exp < currentTime;
  } catch (error) {
    console.error("Error decoding token:", error);
    return true; // Assume expired if there's an error
  }
};

export const getAccessToken = () => {
  const access_token = Cookies.get("access_token");
  return access_token;
};
