import { PUBLIC_API_URL } from "$env/static/public";

export const setImageBase64Url = (image: File | null) => {
  return new Promise<string>((resolve, reject) => {
    if (!image) {
      resolve("");
      return;
    }
    const reader = new FileReader();
    reader.onloadend = () => {
      resolve(reader.result as string);
    };
    reader.onerror = reject;
    reader.readAsDataURL(image);
  });
};

export const getImageBase64Url = (image_id: number) => {
  return fetch(`${PUBLIC_API_URL}/flashcards/images/${image_id}`).then(async (response) => {
    const blob = await response.blob();
    return new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        resolve(reader.result as string);
      };
      reader.onerror = reject;
      reader.readAsDataURL(blob);
    });
  });
};
