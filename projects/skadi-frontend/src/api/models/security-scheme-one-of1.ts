// tslint:disable
/**
 * 调位置
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
 * @interface SecuritySchemeOneOf1
 */
export interface SecuritySchemeOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf1
     */
    bearerFormat?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf1
     */
    description?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf1
     */
    scheme: string;
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf1
     */
    type: SecuritySchemeOneOf1TypeEnum;
}

/**
    * @export
    * @enum {string}
    */
export enum SecuritySchemeOneOf1TypeEnum {
    Http = 'http'
}



