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


import { OAuth2Flows } from './oauth2-flows';

/**
 * 
 * @export
 * @interface SecuritySchemeOneOf2
 */
export interface SecuritySchemeOneOf2 {
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf2
     */
    description?: string | null;
    /**
     * 
     * @type {OAuth2Flows}
     * @memberof SecuritySchemeOneOf2
     */
    flows: OAuth2Flows;
    /**
     * 
     * @type {string}
     * @memberof SecuritySchemeOneOf2
     */
    type: SecuritySchemeOneOf2TypeEnum;
}

/**
    * @export
    * @enum {string}
    */
export enum SecuritySchemeOneOf2TypeEnum {
    Oauth2 = 'oauth2'
}



