import type {PackageInfo, PackageQueryByName} from "./models";
import request from "./request.ts";

export async function packageQueryByName(data: PackageQueryByName) {
    return request.post<PackageInfo>('/package/find', data);
}
