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


import { AnyType } from './any-type';
import { Model154945957 } from './model154945957';
import { Model154945960 } from './model154945960';

/**
 * Describes a single operation parameter.  A unique parameter is defined by a combination of a name and location.
 * @export
 * @interface ParameterOneOf2
 */
export interface ParameterOneOf2 {
    /**
     * Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
     * @type {AnyType}
     * @memberof ParameterOneOf2
     */
    deprecated?: AnyType;
    /**
     * A brief description of the parameter. This could contain examples of use. CommonMark syntax MAY be used for rich text representation.
     * @type {AnyType}
     * @memberof ParameterOneOf2
     */
    description?: AnyType;
    /**
     * 
     * @type {{ [key: string]: Model154945960; }}
     * @memberof ParameterOneOf2
     */
    examples?: { [key: string]: Model154945960; };
    /**
     * 
     * @type {AnyType}
     * @memberof ParameterOneOf2
     */
    explode?: AnyType;
    /**
     * 
     * @type {string}
     * @memberof ParameterOneOf2
     */
    _in: ParameterOneOf2InEnum;
    /**
     * REQUIRED. The name of the parameter. Parameter names are case sensitive. If in is \"path\", the name field MUST correspond to the associated path segment from the path field in the Paths Object. See Path Templating for further information.  If in is \"header\" and the name field is \"Accept\", \"Content-Type\" or \"Authorization\", the parameter definition SHALL be ignored.  For all other cases, the name corresponds to the parameter name used by the in property.
     * @type {string}
     * @memberof ParameterOneOf2
     */
    name: string;
    /**
     * Determines whether this parameter is mandatory. If the parameter location is \"path\", this property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included and its default value is false.
     * @type {boolean}
     * @memberof ParameterOneOf2
     */
    required?: boolean;
    /**
     * 
     * @type {Model154945957}
     * @memberof ParameterOneOf2
     */
    style?: Model154945957;
}

/**
    * @export
    * @enum {string}
    */
export enum ParameterOneOf2InEnum {
    Path = 'path'
}



