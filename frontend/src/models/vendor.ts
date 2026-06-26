import {prettifyDate} from "@/lib/utils.ts";

export type Vendor = {
    id: number;
    name: string;
    date_edited: string;
    date_created: string;
}

export function fixVendorDates(vendor: Vendor): Vendor {
    return {
        id: vendor.id,
        name: vendor.name,
        date_edited: prettifyDate(vendor.date_edited),
        date_created: prettifyDate(vendor.date_created)
    }
}
