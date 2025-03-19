interface Resource {
  id: string;

  namespace?: string;
  oriFilename: string;

  contentType: string;

  deleteTime?: string; // Using ISO 8601 format for DateTime
  insertTime: string; // Using ISO 8601 format for DateTime
}