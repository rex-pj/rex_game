import { ScoringApi } from "$lib/api/scoringApi";
import { UserServerApiOptions } from "$lib/api/apiOptions";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, cookies }) => {
  const scoringApi = new ScoringApi(new UserServerApiOptions(cookies));

  // Try to get user achievements (with unlock status), fallback to public list
  let achievements;
  try {
    achievements = await scoringApi.getMyAchievements(fetch);
  } catch {
    try {
      achievements = await scoringApi.getAchievements(fetch);
    } catch (error) {
      console.error("Failed to fetch achievements:", error);
      achievements = [];
    }
  }

  return {
    achievements: achievements || [],
  };
};
