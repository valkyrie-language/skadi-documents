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


import { ReferenceOrForExample } from './reference-or-for-example';
import { ReferenceOrForHeader } from './reference-or-for-header';
import { ReferenceOrForLink } from './reference-or-for-link';
import { ReferenceOrForMapOfReferenceOrForPathItem } from './reference-or-for-map-of-reference-or-for-path-item';
import { ReferenceOrForParameter } from './reference-or-for-parameter';
import { ReferenceOrForPathItem } from './reference-or-for-path-item';
import { ReferenceOrForRequestBody } from './reference-or-for-request-body';
import { ReferenceOrForResponse } from './reference-or-for-response';
import { ReferenceOrForSecurityScheme } from './reference-or-for-security-scheme';
import { SchemaObject } from './schema-object';

/**
 * Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
 * @export
 * @interface Components
 */
export interface Components {
    [key: string]: object | any;

    /**
     * An object to hold reusable Callback Objects.
     * @type {{ [key: string]: ReferenceOrForMapOfReferenceOrForPathItem; }}
     * @memberof Components
     */
    callbacks?: { [key: string]: ReferenceOrForMapOfReferenceOrForPathItem; };
    /**
     * An object to hold reusable Example Objects.
     * @type {{ [key: string]: ReferenceOrForExample; }}
     * @memberof Components
     */
    examples?: { [key: string]: ReferenceOrForExample; };
    /**
     * An object to hold reusable Header Objects.
     * @type {{ [key: string]: ReferenceOrForHeader; }}
     * @memberof Components
     */
    headers?: { [key: string]: ReferenceOrForHeader; };
    /**
     * An object to hold reusable Link Objects.
     * @type {{ [key: string]: ReferenceOrForLink; }}
     * @memberof Components
     */
    links?: { [key: string]: ReferenceOrForLink; };
    /**
     * An object to hold reusable Parameter Objects.
     * @type {{ [key: string]: ReferenceOrForParameter; }}
     * @memberof Components
     */
    parameters?: { [key: string]: ReferenceOrForParameter; };
    /**
     * An object to hold reusable Path Item Objects.
     * @type {{ [key: string]: ReferenceOrForPathItem; }}
     * @memberof Components
     */
    pathItems?: { [key: string]: ReferenceOrForPathItem; };
    /**
     * An object to hold reusable Request Body Objects.
     * @type {{ [key: string]: ReferenceOrForRequestBody; }}
     * @memberof Components
     */
    requestBodies?: { [key: string]: ReferenceOrForRequestBody; };
    /**
     * An object to hold reusable Response Objects.
     * @type {{ [key: string]: ReferenceOrForResponse; }}
     * @memberof Components
     */
    responses?: { [key: string]: ReferenceOrForResponse; };
    /**
     * An object to hold reusable Schema Objects.
     * @type {{ [key: string]: SchemaObject; }}
     * @memberof Components
     */
    schemas?: { [key: string]: SchemaObject; };
    /**
     * An object to hold reusable Security Scheme Objects.
     * @type {{ [key: string]: ReferenceOrForSecurityScheme; }}
     * @memberof Components
     */
    securitySchemes?: { [key: string]: ReferenceOrForSecurityScheme; };
}


