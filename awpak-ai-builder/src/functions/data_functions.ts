import { DataToAgentHistoryReplace, DataToAgentHistoryReplaceFirst, DataToAgentHistoryReplaceItem, DataToAgentHistoryReplaceLast, DataToAgentHistoryStringToFirst, DataToAgentHistoryStringToItem, DataToAgentHistoryStringToLast, DataToAgentHistoryVariant, type DataToAgentHistory } from "../model/agent_history_mut";
import { DataFromVariant, DataOperationAdd, DataOperationLen, DataOperationStringSplit, DataOperationSubstract, DataOperationVariant, FromAgentHistory, FromAgentHistoryContentFirst, FromAgentHistoryContentFirstMessage, FromAgentHistoryContentFull, FromAgentHistoryContentFullMessages, FromAgentHistoryContentItem, FromAgentHistoryContentItemMessage, FromAgentHistoryContentLast, FromAgentHistoryContentLastMessage, FromAgentHistoryContentRange, FromAgentHistoryContentRangeMessages, FromAgentHistoryContentVariant, FromConcat, FromContext, FromInput, FromNull, FromOperation, FromParsedInput, FromStatic, FromStore, type DataFrom, type DataOperation, type FromAgentHistoryContent } from "../model/data";
import { DataComparatorAnd, DataComparatorEmpty, DataComparatorEq, DataComparatorFalse, DataComparatorGt, DataComparatorLt, DataComparatorNand, DataComparatorNot, DataComparatorNotEmpty, DataComparatorNotEq, DataComparatorOr, DataComparatorRegex, DataComparatorTrue, DataComparatorVariant, DataComparatorXor, type DataComparator } from "../model/data_comparator";
import { GeminiStoreModel, OllamaStoreModel, OpenAIStoreModel, StoreDocumentPdf, StoreDocumentSizerChars, StoreDocumentSizerMarkdown, StoreDocumentSizerNone, StoreDocumentSizerVariant, StoreDocumentText, StoreDocumentVariant, StoreModelVariant, StoreProvider, type StoreDocument, type StoreDocumentSizer, type StoreModel } from "../model/store";
import { WebClientBodyForm, WebClientBodyJson, WebClientBodyVariant, type WebClientBody } from "../model/web_client";
import { is_type_in_enum } from "./form_utils";

export function json_stringify( obj : any ) : string
{
    return JSON.stringify( obj, (_, v) => v === undefined ? null : v );
}

export function is_empty( data : any ) : boolean
{
    return typeof( data ) === "undefined" || data === null;
}

export function not_empty_or_string_eq( data : any, key : string ) : boolean
{
    if( typeof( data ) === "undefined" || data === null ) return false;

    if( ! is_empty( data?.[ key ] ) ) return true;

    if( typeof( data ) === "string" && data == key ) return true;

    return false;
}

export function new_data_operation_variant( old : DataOperation, new_variant : string ) : DataOperation | undefined
{
    if( ! is_type_in_enum( DataOperationVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataOperationVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataOperationVariant.Len )
    {
        return new DataOperationLen();
    }
    else if( new_variant == DataOperationVariant.Add )
    {
        return new DataOperationAdd();
    }
    else if( new_variant == DataOperationVariant.Substract )
    {
        return new DataOperationSubstract();
    }
    else if( new_variant == DataOperationVariant.StringSplit )
    {
        return new DataOperationStringSplit();
    }

    return undefined;
}

export function new_store_provider_variant( old : StoreProvider, new_variant : string ) : StoreProvider | undefined
{
    if( ! is_type_in_enum( StoreProvider, new_variant ) ) { return undefined; }

    new_variant = new_variant as StoreProvider;

    if( old == new_variant ) { return old; }

    return new_variant as StoreProvider;
}

export function new_store_document_variant( old : StoreDocument, new_variant : string ) : StoreDocument | undefined
{
    if( ! is_type_in_enum( StoreDocumentVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as StoreDocumentVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == StoreDocumentVariant.Text )
    {
        let ret = new StoreDocumentText();

        ret.path = old.path;
        ret.sizer = old.sizer;

        return ret;
    }
    else if( new_variant == StoreDocumentVariant.Pdf )
    {
        let ret = new StoreDocumentPdf();

        ret.path = old.path;
        ret.sizer = old.sizer;

        return ret;
    }
}

export function new_store_document_sizer_variant( old : StoreDocumentSizer, new_variant : string ) : StoreDocumentSizer | undefined
{
    if( ! is_type_in_enum( StoreDocumentSizerVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as StoreDocumentSizerVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == StoreDocumentSizerVariant.Chars )
    {
        let ret = new StoreDocumentSizerChars();

        if( old._variant == StoreDocumentSizerVariant.Markdown )
        {
            ret.desired = old.desired;
            ret.max = old.max;
        }

        return ret;
    }
    else if( new_variant == StoreDocumentSizerVariant.Markdown )
    {
        let ret = new StoreDocumentSizerMarkdown();

        if( old._variant == StoreDocumentSizerVariant.Chars )
        {
            ret.desired = old.desired;
            ret.max = old.max;
        }

        return ret;
    }
    else if( new_variant == StoreDocumentSizerVariant.None )
    {
        return new StoreDocumentSizerNone();
    }
}

export function new_store_model_variant( old : StoreModel, new_variant : string ) : StoreModel | undefined
{
    if( ! is_type_in_enum( StoreModelVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as StoreModelVariant;

    if( old._variant == new_variant ) { return old; }

    let api_key = ( old._variant == StoreModelVariant.OpenAI || old._variant == StoreModelVariant.Gemini ) ? old.api_key : "";

    if( new_variant == StoreModelVariant.OpenAI )
    {
        let ret = new OpenAIStoreModel();

        ret.model = old.model;

        ret.api_key = api_key;

        return ret;
    }
    else if( new_variant == StoreModelVariant.Gemini )
    {
        let ret = new GeminiStoreModel();

        ret.model = old.model;

        ret.api_key = api_key;
        
        return ret;
    }
    else if( new_variant == StoreModelVariant.Ollama )
    {
        let ret = new OllamaStoreModel();

        ret.model = old.model;
        
        return ret;
    }
}

export function new_data_comparator_variant( old : DataComparator, new_variant : string ) : DataComparator | undefined
{
    if( ! is_type_in_enum( DataComparatorVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataComparatorVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataComparatorVariant.Eq )
    {
        return new DataComparatorEq();
    }
    else if( new_variant == DataComparatorVariant.NotEq )
    {
        return new DataComparatorNotEq();
    }
    else if( new_variant == DataComparatorVariant.Gt )
    {
        return new DataComparatorGt();
    }
    else if( new_variant == DataComparatorVariant.Lt )
    {
        return new DataComparatorLt();
    }
    else if( new_variant == DataComparatorVariant.Regex )
    {
        return new DataComparatorRegex()
    }
    else if( new_variant == DataComparatorVariant.And )
    {
        return new DataComparatorAnd();
    }
    else if( new_variant == DataComparatorVariant.Or )
    {
        return new DataComparatorOr();
    }
    else if( new_variant == DataComparatorVariant.Xor )
    {
        return new DataComparatorXor();
    }
    else if( new_variant == DataComparatorVariant.Nand )
    {
        return new DataComparatorNand();
    }
    else if( new_variant == DataComparatorVariant.Not )
    {
        return new DataComparatorNot();
    }
    else if( new_variant == DataComparatorVariant.Empty )
    {
        return new DataComparatorEmpty();
    }
    else if( new_variant == DataComparatorVariant.NotEmpty )
    {
        return new DataComparatorNotEmpty();
    }
    else if( new_variant == DataComparatorVariant.True )
    {
        return new DataComparatorTrue();
    }
    else if( new_variant == DataComparatorVariant.False )
    {
        return new DataComparatorFalse();
    }

    return undefined;
}

export function new_body_variant( old : WebClientBody, new_variant : string ) : WebClientBody | undefined
{
    if( ! is_type_in_enum( WebClientBodyVariant, new_variant ) ) { return undefined; }

    if( ! old ) old = new WebClientBodyJson();

    new_variant = new_variant as WebClientBodyVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == WebClientBodyVariant.Json )
    {
        return new WebClientBodyJson();
    }
    else if( new_variant == WebClientBodyVariant.Form )
    {
        return new WebClientBodyForm();
    }
}

export function new_data_to_agent_history( old : DataToAgentHistory, new_variant : string ) : DataToAgentHistory | undefined
{
    if( ! is_type_in_enum( DataToAgentHistoryVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataToAgentHistoryVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataToAgentHistoryVariant.Replace )
    {
        return new DataToAgentHistoryReplace();
    }
    else if( new_variant == DataToAgentHistoryVariant.ReplaceFirst )
    {
        return new DataToAgentHistoryReplaceFirst();
    }
    else if( new_variant == DataToAgentHistoryVariant.ReplaceLast )
    {
        return new DataToAgentHistoryReplaceLast();
    }
    else if( new_variant == DataToAgentHistoryVariant.ReplaceItem )
    {
        let ret = new DataToAgentHistoryReplaceItem();

        if( old._variant == DataToAgentHistoryVariant.StringToItem )
        {
            ret.value = old.value;
        }

        return ret;
    }
    else if( new_variant == DataToAgentHistoryVariant.StringToLast )
    {
        return new DataToAgentHistoryStringToLast();
    }
    else if( new_variant == DataToAgentHistoryVariant.StringToFirst )
    {
        return new DataToAgentHistoryStringToFirst();
    }
    else if( new_variant == DataToAgentHistoryVariant.StringToItem )
    {
        let ret = new DataToAgentHistoryStringToItem();

        if( old._variant == DataToAgentHistoryVariant.ReplaceItem )
        {
            ret.value = old.value;
        }

        return ret;
    }
}

export function new_data_from_agent_history_content( old : FromAgentHistoryContent, new_variant : string ) : FromAgentHistoryContent | undefined
{
    if( ! is_type_in_enum( FromAgentHistoryContentVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as FromAgentHistoryContentVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == FromAgentHistoryContentVariant.Full )
    {
        return new FromAgentHistoryContentFull();
    }
    else if( new_variant == FromAgentHistoryContentVariant.FullMessages )
    {
        return new FromAgentHistoryContentFullMessages();
    }
    else if( new_variant == FromAgentHistoryContentVariant.First )
    {
        return new FromAgentHistoryContentFirst();
    }
    else if( new_variant == FromAgentHistoryContentVariant.FirstMessage )
    {
        return new FromAgentHistoryContentFirstMessage();
    }
    else if( new_variant == FromAgentHistoryContentVariant.Last )
    {
        return new FromAgentHistoryContentLast();
    }
    else if( new_variant == FromAgentHistoryContentVariant.LastMessage )
    {
        return new FromAgentHistoryContentLastMessage();
    }
    else if( new_variant == FromAgentHistoryContentVariant.Range )
    {
        let ret = new FromAgentHistoryContentRange();

        if( old._variant == FromAgentHistoryContentVariant.RangeMessages )
        {
            ret.from = old.from;
            ret.to = old.to;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.RangeMessages )
    {
        let ret = new FromAgentHistoryContentRangeMessages();

        if( old._variant == FromAgentHistoryContentVariant.Range )
        {
            ret.from = old.from;
            ret.to = old.to;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.Item )
    {
        let ret = new FromAgentHistoryContentItem();

        if( old._variant == FromAgentHistoryContentVariant.ItemMessage )
        {
            ret.value = old.value;
        }

        return ret;
    }
    else if( new_variant == FromAgentHistoryContentVariant.ItemMessage )
    {
        let ret = new FromAgentHistoryContentItemMessage();

        if( old._variant == FromAgentHistoryContentVariant.Item )
        {
            ret.value = old.value;
        }

        return ret;
    }
}

export function new_data_from_variant( old : DataFrom, new_variant : string ) : DataFrom | undefined
{
    if( ! is_type_in_enum( DataFromVariant, new_variant ) ) { return undefined; }

    new_variant = new_variant as DataFromVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == DataFromVariant.Context )
    {
        return new FromContext();
    }
    else if( new_variant == DataFromVariant.Input )
    {
        return new FromInput();
    }
    else if( new_variant == DataFromVariant.ParsedInput )
    {
        return new FromParsedInput();
    }
    else if( new_variant == DataFromVariant.Static )
    {
        return new FromStatic();
    }
    else if( new_variant == DataFromVariant.Operation )
    {
        return new FromOperation();
    }
    else if( new_variant == DataFromVariant.Concat )
    {
        return new FromConcat();
    }
    else if( new_variant == DataFromVariant.Null )
    {
        return new FromNull();
    }
    else if( new_variant == DataFromVariant.AgentHistory )
    {
        return new FromAgentHistory();
    }
    else if( new_variant == DataFromVariant.Store )
    {
        return new FromStore();
    }
    else
    {
        return undefined;
    }
}

export function number_from_any( next : any ) : number | undefined
{
    if( 
        typeof( next ) === "undefined" || 
        next === null ||
        ( typeof( next ) === "string" && next.trim() == "" )
    ) { return undefined; }

    next = Number( next );

    if( typeof( next ) === "undefined" || next === null || isNaN( next ) ) return undefined;

    return next;
}