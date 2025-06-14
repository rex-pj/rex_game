import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = ({ data }) => {
  if (data && data.currentUser) {
    const { currentUser } = data;
    return { currentUser };
  }

  return {};
};
