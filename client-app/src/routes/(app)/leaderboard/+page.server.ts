import { ScoringApi } from "$lib/api/scoringApi";
import { UserServerApiOptions } from "$lib/api/apiOptions";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, cookies }) => {
  const scoringApi = new ScoringApi(new UserServerApiOptions(cookies));

  const [leaderboard, myStats] = await Promise.all([
    scoringApi.getLeaderboard(fetch, 1, 20).catch((error) => {
      console.error("Failed to fetch leaderboard:", error);
      return [];
    }),
    scoringApi.getMyStats(fetch).catch(() => null),
  ]);

  return {
    leaderboard: leaderboard || [],
    myStats,
  };
};
