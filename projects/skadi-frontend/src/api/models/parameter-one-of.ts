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


import { AnyType } from './any-type';
import { Model154945959 } from './model154945959';
import { Model154945960 } from './model154945960';

/**
 * Describes a single operation parameter.  A unique parameter is defined by a combination of a name and location.
 * @export
 * @interface ParameterOneOf
 */
export interface ParameterOneOf {
    /**
     * Sets the ability to pass empty-valued parameters. This is valid only for query parameters and allows sending a parameter with an empty value. Default value is false. If style is used, and if behavior is n/a (cannot be serialized), the value of allowEmptyValue SHALL be ignored.
     * @type {AnyType}
     * @memberof ParameterOneOf
     */
    allow_empty_value?: AnyType;
    /**
     * Determines whether the parameter value SHOULD allow reserved characters, as defined by RFC3986 :/?#[]@!$&\'()*+,;= to be included without percent-encoding. This property only applies to parameters with an in value of query. The default value is false.
     * @type {boolean}
     * @memberof ParameterOneOf
     */
    allow_reserved?: boolean;
    /**
     * Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
     * @type {AnyType}
     * @memberof ParameterOneOf
     */
    deprecated?: AnyType;
    /**
     * A brief description of the parameter. This could contain examples of use. CommonMark syntax MAY be used for rich text representation.
     * @type {AnyType}
     * @memberof ParameterOneOf
     */
    description?: AnyType;
    /**
     * 
     * @type {{ [key: string]: Model154945960; }}
     * @memberof ParameterOneOf
     */
    examples?: { [key: string]: Model154945960; };
    /**
     * 
     * @type {AnyType}
     * @memberof ParameterOneOf
     */
    explode?: AnyType;
    /**
     * 
     * @type {string}
     * @memberof ParameterOneOf
     */
    _in: ParameterOneOfInEnum;
    /**
     * REQUIRED. The name of the parameter. Parameter names are case sensitive. If in is \"path\", the name field MUST correspond to the associated path segment from the path field in the Paths Object. See Path Templating for further information.  If in is \"header\" and the name field is \"Accept\", \"Content-Type\" or \"Authorization\", the parameter definition SHALL be ignored.  For all other cases, the name corresponds to the parameter name used by the in property.
     * @type {string}
     * @memberof ParameterOneOf
     */
    name: string;
    /**
     * Determines whether this parameter is mandatory. If the parameter location is \"path\", this property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included and its default value is false.
     * @type {boolean}
     * @memberof ParameterOneOf
     */
    required?: boolean;
    /**
     * 
     * @type {Model154945959}
     * @memberof ParameterOneOf
     */
    style?: Model154945959;
}

/**
    * @export
    * @enum {string}
    */
export enum ParameterOneOfInEnum {
    Query = 'query'
}



