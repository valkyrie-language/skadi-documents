import type {HomeStatistics} from "./models";
import request from "./request.ts";

export async function homeStatistics() {
    return request.get<HomeStatistics>('/home/statistics');
}