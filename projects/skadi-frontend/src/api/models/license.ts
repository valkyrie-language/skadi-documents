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
 * License information for the exposed API.
 * @export
 * @interface License
 */
export interface License {
    [key: string]: object | any;

    /**
     * An [SPDX](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60) license expression for the API. The `identifier` field is mutually exclusive of the `url` field.
     * @type {string}
     * @memberof License
     */
    identifier?: string | null;
    /**
     * REQUIRED. The license name used for the API.
     * @type {string}
     * @memberof License
     */
    name: string;
    /**
     * A URL to the license used for the API. This MUST be in the form of a URL. The `url` field is mutually exclusive of the `identifier` field.
     * @type {string}
     * @memberof License
     */
    url?: string | null;
}


