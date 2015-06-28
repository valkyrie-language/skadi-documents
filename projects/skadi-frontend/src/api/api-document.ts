import request from "./request.ts";
import type {DocumentInfo, DocumentQueryByPath} from "./models";

export async function documentQueryByPath(data: DocumentQueryByPath) {
    return request.post<DocumentInfo>('/document/query', data);
}
