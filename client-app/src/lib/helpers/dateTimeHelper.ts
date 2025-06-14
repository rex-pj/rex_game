import { format as convert } from "date-fns";

export const standardizeDate = (date: string, format?: string) => {
  if (!date) {
    return "N/A";
  }

  if (format) {
    return convert(date, format);
  }
  return convert(date, "dd/MM/yyyy");
};
