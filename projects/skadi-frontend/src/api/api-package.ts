import type {PackageDetail, PackageQueryByName} from "./models";
import request from "./request.ts";

export async function packageQueryByName(data: PackageQueryByName) {
    return request.post<PackageDetail>('/package/find', data);
}
