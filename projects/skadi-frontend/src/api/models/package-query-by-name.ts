// tslint:disable
/**
 * Skadi Documention Generator
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */



/**
 * 
 * @export
 * @interface PackageQueryByName
 */
export interface PackageQueryByName {
    /**
     * The name of the package.
     * @type {string}
     * @memberof PackageQueryByName
     */
    name: string;
    /**
     * The owner of the package.
     * @type {string}
     * @memberof PackageQueryByName
     */
    organization: string;
    /**
     * The specific version of the package to query.
     * @type {string}
     * @memberof PackageQueryByName
     */
    version?: string | null;
}


