import type {Vendor} from "@/models/vendor.ts";
import type {Material} from "@/models/material.ts";

export type Filament = {
    id: number;
    vendor: Vendor;
    material: Material;
    name: string;
    temp_min: number;
    temp_max: number | null;
    temp_bed_min: number;
    temp_bed_max: number | null;
    price: number;
    date_created: string;
    date_updated: string;
}