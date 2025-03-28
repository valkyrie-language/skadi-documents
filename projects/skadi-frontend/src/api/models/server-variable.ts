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
 * An object representing a Server Variable for server URL template substitution.
 * @export
 * @interface ServerVariable
 */
export interface ServerVariable {
    [key: string]: object | any;

    /**
     * REQUIRED. The default value to use for substitution, and to send, if an alternate value is not supplied. Unlike the Schema Object\'s default, this value MUST be provided by the consumer.
     * @type {string}
     * @memberof ServerVariable
     */
    _default: string;
    /**
     * An optional description for the server variable. CommonMark syntax MAY be used for rich text representation.
     * @type {string}
     * @memberof ServerVariable
     */
    description?: string | null;
    /**
     * An enumeration of string values to be used if the substitution options are from a limited set.
     * @type {Array<string>}
     * @memberof ServerVariable
     */
    _enum?: Array<string>;
}


