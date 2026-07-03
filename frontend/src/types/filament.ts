import type {Material} from "./material.ts";
import type {Vendor} from "./vendor.ts";

export type Filament = {
    id: number;
    material: Material;
    vendor: Vendor;
    name: string;
    temp_min: number;
    temp_max?: number;
    temp_bed_min: number;
    temp_bed_max?: number;
    price: number;
    date_created: string;
    date_updated: string;
}