import { get, writable, type Writable } from "svelte/store";
import { FlashcardService } from "$lib/services/flashcardService";
import Cookies from "js-cookie";

const flashcardService: FlashcardService = new FlashcardService(Cookies);
export const flashcards: Writable<any[]> = writable([]);
export const currentPage = writable(1);
export const totalPages = writable(1);
const itemsPerPage = 10;

// Fetch flashcards data (mocked for now)
export const fetchFlashcards = async (page: number) => {
  // Replace this with your API call
  const response = await flashcardService.getList(fetch, page, itemsPerPage);

  // const start = (page - 1) * itemsPerPage;
  // const end = start + itemsPerPage;
  // flashcards.set(mockData.slice(start, end));
  // totalPages.set(Math.ceil(totalItems / itemsPerPage));
};

export const changePage = (page: number) => {
  if (page >= 1 && page <= get(totalPages)) {
    currentPage.set(page);
    fetchFlashcards(page);
  }
};
