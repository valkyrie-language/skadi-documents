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


import { MediaType } from './media-type';
import { ReferenceOrForHeader } from './reference-or-for-header';
import { ReferenceOrForLink } from './reference-or-for-link';

/**
 * 
 * @export
 * @interface Response
 */
export interface Response {
    [key: string]: object | any;

    /**
     * A map containing descriptions of potential response payloads. The key is a media type or media type range and the value describes it. For responses that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/_*
     * @type {{ [key: string]: MediaType; }}
     * @memberof Response
     */
    content?: { [key: string]: MediaType; };
    /**
     * REQUIRED. A description of the response. CommonMark syntax MAY be used for rich text representation.
     * @type {string}
     * @memberof Response
     */
    description: string;
    /**
     * Maps a header name to its definition. RFC7230 states header names are case insensitive. If a response header is defined with the name \"Content-Type\", it SHALL be ignored.
     * @type {{ [key: string]: ReferenceOrForHeader; }}
     * @memberof Response
     */
    headers?: { [key: string]: ReferenceOrForHeader; };
    /**
     * A map of operations links that can be followed from the response. The key of the map is a short name for the link, following the naming constraints of the names for Component Objects.
     * @type {{ [key: string]: ReferenceOrForLink; }}
     * @memberof Response
     */
    links?: { [key: string]: ReferenceOrForLink; };
}


