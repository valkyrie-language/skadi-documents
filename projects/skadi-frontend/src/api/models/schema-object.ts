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
import { Model154945943 } from './model154945943';
import { Null } from './null';
import { Schema } from './schema';
import { SchemaObject2 } from './schema-object2';
import { SingleOrVecForInstanceType } from './single-or-vec-for-instance-type';
import { SingleOrVecForSchema } from './single-or-vec-for-schema';

/**
 * A JSON Schema.
 * @export
 * @interface SchemaObject
 */
export interface SchemaObject {
    /**
     * A free-form property to include an example of an instance for this schema. To represent examples that cannot be naturally represented in JSON or YAML, a string value can be used to contain the example with escaping where necessary. **Deprecated:** The `example` property has been deprecated in favor of the JSON Schema `examples` keyword. Use of `example` is discouraged, and later versions of this specification may remove it.
     * @type {AnyType}
     * @memberof SchemaObject
     */
    example?: AnyType;
    /**
     * Additional external documentation for this schema.
     * @type {Model154945943 | Null}
     * @memberof SchemaObject
     */
    externalDocs?: Model154945943 | Null;
}


