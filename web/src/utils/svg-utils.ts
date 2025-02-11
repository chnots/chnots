import request from "./request";

export async function fetchAndModifySvg(url: string) {
  try {
    const response = await request.get(url);
    const svgString = response as string;

    //Using DOMParser for robust attribute removal
    const parser = new DOMParser();
    const doc = parser.parseFromString(svgString, "image/svg+xml");
    const svgElement = doc.documentElement;
    svgElement.removeAttribute("width");
    svgElement.removeAttribute("height");

    const modifiedSvgString = new XMLSerializer().serializeToString(doc);

    return modifiedSvgString;
  } catch (error) {
    console.error("Error fetching or modifying SVG:", error);
    return null; // Or throw the error, depending on your error handling strategy
  }
}
