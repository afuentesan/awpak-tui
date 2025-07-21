import { FromStatic, type DataFrom } from "./data";

export class WebClient
{
    url : DataFrom = new FromStatic();
    method : AwpakMethod = AwpakMethod.Get;
    headers : Array<WebClientNameValue> = [];
    query_params : Array<WebClientNameValue> = [];
    body : WebClientBody | undefined;
    output : Array<WebClientOutput> = [];
    timeout : number | undefined;
}

export enum AwpakMethod
{
    Options = "Options",
    Get = "Get",
    Post = "Post",
    Put = "Put",
    Delete = "Delete",
    Head = "Head",
    Trace = "Trace",
    Connect = "Connect",
    Patch = "Patch",
}

export class WebClientNameValue
{
    name : DataFrom = new FromStatic();
    value : DataFrom = new FromStatic();
}

export type WebClientBody = WebClientBodyJson | WebClientBodyForm;

export enum WebClientBodyVariant
{
    Json = "Json",
    Form = "Form"
}

export class WebClientBodyJson
{
    readonly _variant = WebClientBodyVariant.Json;

    value : DataFrom = new FromStatic();
}

export class WebClientBodyForm
{
    readonly _variant = WebClientBodyVariant.Form;

     value : Array<WebClientNameValue> = [];
}

export type WebClientOutput = WebClientOutputVersion |
                              WebClientOutputStatus |
                              WebClientOutputHeader |
                              WebClientOutputBody |
                              WebClientOutputObject;

export enum WebClientOutputVariant
{
    Version = "Version",
    Status = "Status",
    Header = "Header",
    Body = "Body",
    Object = "Object"
}

export class WebClientOutputVersion
{
    readonly _variant = WebClientOutputVariant.Version;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class WebClientOutputStatus
{
    readonly _variant = WebClientOutputVariant.Status;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class WebClientOutputHeader
{
    readonly _variant = WebClientOutputVariant.Header;

    name : string = "";

    prefix : string | undefined;
    suffix : string | undefined;
}

export class WebClientOutputBody
{
    readonly _variant = WebClientOutputVariant.Body;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class WebClientOutputObject
{
    readonly _variant = WebClientOutputVariant.Object;

    prefix : string | undefined;
    suffix : string | undefined;
}