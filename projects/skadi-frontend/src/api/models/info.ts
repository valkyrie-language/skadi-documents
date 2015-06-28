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


import { Contact } from './contact';
import { License } from './license';
import { Null } from './null';

/**
 * The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
 * @export
 * @interface Info
 */
export interface Info {
    [key: string]: object | any;

    /**
     * The contact information for the exposed API.
     * @type {Contact | Null}
     * @memberof Info
     */
    contact?: Contact | Null;
    /**
     * A description of the API. CommonMark syntax MAY be used for rich text representation.
     * @type {string}
     * @memberof Info
     */
    description?: string | null;
    /**
     * The license information for the exposed API.
     * @type {License | Null}
     * @memberof Info
     */
    license?: License | Null;
    /**
     * A short summary of the API.
     * @type {string}
     * @memberof Info
     */
    summary?: string | null;
    /**
     * A URL to the Terms of Service for the API. This MUST be in the format of a URL.
     * @type {string}
     * @memberof Info
     */
    termsOfService?: string | null;
    /**
     * REQUIRED. The title of the application.
     * @type {string}
     * @memberof Info
     */
    title: string;
    /**
     * REQUIRED. The version of the OpenAPI document (which is distinct from the OpenAPI Specification version or the API implementation version).
     * @type {string}
     * @memberof Info
     */
    version: string;
}


