import {
    Button, ButtonGroup, Drawer,
    InputGroup, type Key,
    Label,
    ListBox,
    Select,
    Switch,
    TextField,
    toast,
    Typography
} from "@heroui/react";
import {useParams} from "react-router";
import {type Dispatch, type SetStateAction, useEffect, useState} from "react";
import {type Filament} from "../types/filament.ts";
import axios from "axios";
import {BASE_URL} from "../main.tsx";
import FilamentNotFound from "./FilamentNotFound.tsx";
import type {Vendor} from "../types/vendor.ts";
import type {Material} from "../types/material.ts";
import type {Color} from "../types/color.ts";
import {
    CheckIcon, ChevronDownIcon, ChevronUpIcon, CircleQuestionMarkIcon,
    PaintbrushIcon,
    PencilIcon, SpoolIcon,
    ThermometerIcon, TrashIcon,
    WeightIcon,
    XIcon
} from "lucide-react";
import type {Weight} from "../types/weight.ts";

export default function Filament() {
    const [filament, setFilament] = useState<Filament | undefined>(undefined)
    const [returnNotFound, setReturnNotFound] = useState<boolean>(false)

    const [vendors, setVendors] = useState<Vendor[]>([])
    const [materials, setMaterials] = useState<Material[]>([])

    // params & stuff
    const paramsID = useParams().id;
    if (paramsID === undefined) {
        return <FilamentNotFound cause={"invalidID"}/>
    }

    // params2 & stuff
    const ID = Number(paramsID);
    if (isNaN(ID)) {
        return <FilamentNotFound cause={"invalidID"} id={paramsID}/>
    }

    // load
    useEffect(() => {
        if (ID === undefined || isNaN(ID)) return
        axios.get<Filament>(`${BASE_URL}/filament/${ID}`)
            .then(resp => {
                setFilament(resp.data)
            })
            .catch(e => {
                console.error(e)
                toast.danger(`Error while getting Filament`, {
                    description: `${e}`
                })
                setReturnNotFound(true)
            })

        axios.get<Vendor[]>(`${BASE_URL}/vendor`)
            .then(resp => setVendors(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Error while getting Vendors", {
                    description: `${e}`
                })
            })

        axios.get<Material[]>(`${BASE_URL}/material`)
            .then(resp => setMaterials(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Error while getting Materials", {
                    description: `${e}`
                })
            })
    }, [ID]);

    if (returnNotFound) {
        return <FilamentNotFound cause={"notFound"} id={ID}/>
    }

    if (filament === undefined) {
        return (
            <main>loading...</main>
        )
    }

    // const filamentColorGradient = () => {
    //     let c = filament.colors.map(c => (`, #${c.hex}`))
    //     return `linear-gradient(to left ${c} )`
    // }

    return (
        <main>
            <Typography type={"h2"}>{filament.name}</Typography>
            {
                filament.colors.length > 0
                ? <div
                        className={"w-full h-2 rounded-full mt-4 mb-8 bg-linear-to-r shadow"}
                        style={{backgroundImage: `linear-gradient(to right,  ${filament.colors.map(c => `#${c.hex}`).join(", ")})`}}
                 />
                : <div className={"h-2 mt-4 mb-8"}/>
            }

            <div className={"grid grid-cols-4 gap-12 *:flex *:flex-col *:gap-8 pb-8 border-b"}>
                <div className={"gap-8"}>
                    <div className={"flex items-center gap-1.5 -mb-4"}>
                        <SpoolIcon className={"size-7"}/>
                        <Typography type={"h3"}>General Info</Typography>
                    </div>
                    <GeneralSelect
                        filament_id={ID}
                        defaultSelectedID={filament.vendor.id}
                        items={vendors}
                        type={"Vendor"}
                    />
                    <GeneralSelect
                        filament_id={ID}
                        defaultSelectedID={filament.material.id}
                        items={materials}
                        type={"Material"}
                    />
                    <PriceSection filament_id={ID} price={filament.price}/>
                </div>
                <div>
                    <div className={"flex items-center gap-1.5 -mb-4"}>
                        <WeightIcon className={"size-7"}/>
                        <Typography type={"h3"}>Weight</Typography>
                    </div>
                    <WeightSection weight={filament.weight}/>
                </div>
                <div>
                    <div className={"flex items-center gap-1 -mb-4"}>
                        <ThermometerIcon className={"size-7"}/>
                        <Typography type={"h3"}>Temperature</Typography>
                    </div>
                    <GeneralTempSection type={"Nozzle"} min={filament.temp.min} max={filament.temp.max}/>
                    <GeneralTempSection type={"Bed"} min={filament.temp.bed_min} max={filament.temp.bed_max}/>
                </div>
                <div>
                    <div className={"flex items-center gap-1.5 -mb-4"}>
                        <PaintbrushIcon className={"size-7"}/>
                        <Typography type={"h3"}>Colors</Typography>
                    </div>
                    <ColorsSection filament={filament} setFilament={setFilament}/>
                </div>
            </div>
            <div className={"pt-4"}>
                <Typography type={"h2"}>Images</Typography>
                //TODO
            </div>
        </main>
    )
}

// type SectionWithIconProps = {
//     icon: LucideIcon;
//     children: ReactNode;
// }
//
// function SectionWithIcon(props: SectionWithIconProps) {
//     return (
//         <div className={"flex gap-3"}>
//             <props.icon className={"size-16 shrink-0"} strokeWidth={1.25}/>
//             {props.children}
//         </div>
//     )
// }

type GeneralSelectProps = {
    filament_id: number;
    defaultSelectedID: number;
    items: Vendor[] | Material[];
    type: "Vendor" | "Material"
}

function GeneralSelect(props: GeneralSelectProps) {
    // TODO crud
    return (
        <Select className={"w-full"} defaultValue={props.defaultSelectedID} isRequired={true}>
            <Label>{props.type}</Label>
            <Select.Trigger>
                <Select.Value/>
                <Select.Indicator/>
            </Select.Trigger>
            <Select.Popover>
                <ListBox>
                    {
                        props.items.map(i => (
                            <ListBox.Item id={i.id} textValue={i.name}>
                                {i.name}
                                <ListBox.ItemIndicator/>
                            </ListBox.Item>
                        ))
                    }
                </ListBox>
            </Select.Popover>
        </Select>
    )
}

type PriceSectionProps = {
    filament_id: number;
    price: number;
}

function PriceSection(props: PriceSectionProps) {
    const [editing, setEditing] = useState<boolean>(false)
    const [value, setValue] = useState<number>(props.price)
    // TODO crud
    // TODO nejak nech to vyzera normalne

    const cancel = () => {
        setEditing(false)
        setValue(props.price)
    }

    return (
        <TextField className={"w-full"} isRequired={true}>
            <Label>Price</Label>
            <InputGroup>
                <InputGroup.Input
                    value={value}
                    onChange={e => setValue(e.target.value)}
                    disabled={!editing}
                    type={"number"}
                />
                <InputGroup.Suffix className={"flex gap-2.5 *:cursor-pointer *:size-5"}>
                    {
                        !editing
                            ? <>
                                <PencilIcon onClick={() => setEditing(true)}/>
                            </>
                            : <>
                                <CheckIcon
                                    className={"clickable-no-scale"}
                                />
                                <XIcon
                                    className={"clickable-no-scale"}
                                    onClick={cancel}
                                />
                            </>

                    }
                </InputGroup.Suffix>
            </InputGroup>
        </TextField>
    )
}

type WeightSectionProps = {
    weight: Weight;
}

function WeightSection(props: WeightSectionProps) {
    const [originalWeight, setOriginalWeight] = useState<number>(props.weight.original)
    const [netWeight, setNetWeight] = useState<number>(props.weight.net)
    const [spoolWeight, setSpoolWeight] = useState<number>(props.weight.spool)
    const [bruttoWeight, setBruttoWeight] = useState<number>(props.weight.net + props.weight.spool)

    return (
        <>
            <TextField className={"flex flex-col gap-4 *:flex *:flex-col *:gap-1"}>
                <div>
                    <Label>Original Weight</Label>
                    <InputGroup>
                        <InputGroup.Input/>
                        <InputGroup.Suffix>g</InputGroup.Suffix>
                    </InputGroup>
                </div>
                <div>
                    <Label>Net Weight</Label>
                    <InputGroup>
                        <InputGroup.Input/>
                        <InputGroup.Suffix>g</InputGroup.Suffix>
                    </InputGroup>
                </div>
                <div>
                    <Label>Brutto Weight</Label>
                    <InputGroup>
                        <InputGroup.Input/>
                        <InputGroup.Suffix>g</InputGroup.Suffix>
                    </InputGroup>
                </div>
                <div>
                    <Label>Spool Weight</Label>
                    <InputGroup>
                        <InputGroup.Input/>
                        <InputGroup.Suffix>g</InputGroup.Suffix>
                    </InputGroup>
                </div>
                <Drawer>
                    <Button variant={"outline"} size={"sm"}>
                        <div className={"flex gap-2 items-center"}>
                            <CircleQuestionMarkIcon/>
                            What are these values?
                        </div>
                    </Button>
                    <Drawer.Backdrop>
                        <Drawer.Content placement={"right"}>
                            <Drawer.Dialog>
                                <Drawer.Header>
                                    <Drawer.Heading>Filament Weight</Drawer.Heading>
                                </Drawer.Header>
                                <Drawer.Body>
                                    <p>//TODO</p>
                                </Drawer.Body>
                                <Drawer.Footer>
                                    <Button slot={"close"} variant={"tertiary"}>Close</Button>
                                </Drawer.Footer>
                            </Drawer.Dialog>
                        </Drawer.Content>
                    </Drawer.Backdrop>
                </Drawer>
            </TextField>
        </>
    )
}

type GeneralTempSectionProps = {
    type: "Nozzle" | "Bed";
    min: number;
    max: number | null | undefined;

}

function GeneralTempSection(props: GeneralTempSectionProps) {
    const [advanced, setAdvanced] = useState<boolean>(props.max !== null)
    const [minValue, setMinValue] = useState<number | null>(props.min)
    const [maxValue, setMaxValue] = useState<number | null | undefined>(props.max)

    // TODO crud

    return (
        <TextField className={"flex flex-col gap-2"}>
            <Label>{props.type} Temperature</Label>
            <Switch isSelected={advanced} onChange={setAdvanced}>
                <Switch.Content>
                    <Switch.Control>
                        <Switch.Thumb/>
                    </Switch.Control>
                    Advanced
                </Switch.Content>
            </Switch>
            <div className={"flex gap-2 items-center"}>
                <InputGroup className={"w-full"}>
                    <InputGroup.Input
                        value={minValue || undefined}
                        type={"number"}
                        onChange={e => setMinValue(e.target.value)}
                        placeholder={"Min value"}
                        className={"w-full"}
                    />
                    <InputGroup.Suffix>°C</InputGroup.Suffix>
                </InputGroup>
                {
                    // TODO fix size
                    advanced && <>
                        <p> to </p>
                        <InputGroup>
                            <InputGroup.Input
                                value={maxValue || undefined}
                                type={"number"}
                                onChange={e => setMaxValue(e.target.value)}
                                placeholder={"Max value"}
                                className={"w-full"}
                            />
                            <InputGroup.Suffix>°C</InputGroup.Suffix>
                        </InputGroup>
                    </>
                }
            </div>
        </TextField>
    )
}

type ColorsSectionProps = {
    filament: Filament;
    setFilament: Dispatch<SetStateAction<Filament>>
}

function ColorsSection(props: ColorsSectionProps) {
    const [allColors, setAllColors] = useState<Color[]>([])
    const [assignableColors, setAssignableColors] = useState<Color[]>([])

    // TODO crud

    useEffect(() => {
        axios.get<Color[]>(`${BASE_URL}/color`)
            .then(resp => setAllColors(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Error while getting Colors", {
                    description: `${e}`
                })
            })
    }, []);

    useEffect(() => {
        const result =  allColors.filter(c => (
            !props.filament.colors
                .map(x => x.id)
                .includes(c.id)
        ));
        setAssignableColors(result)
    }, [allColors, props.filament]);

    function unassignColorClicked(color_id: number) {
        axios.delete<Color>(`${BASE_URL}/filament/${props.filament.id}/color/${color_id}`)
            .then(resp => {
                props.setFilament(prev => ({
                    ...prev,
                    colors: prev.colors.filter(c => c.id !== resp.data.id)
                }))
            })
            .catch(e => {
                console.error(e)
                toast.danger("Failed unassigning Color from Filament", {
                    description: `${e}`
                })
            })
    }

    function onAssignChange(value: Key | null) {
        if (value === null) return
        const selected = assignableColors.find(c => c.hex === value)
        if (!selected) return

        axios.post(`${BASE_URL}/filament_color`, {
            filament_id: props.filament.id,
            color_id: selected.id
        })
            .then(() => {
                props.setFilament(prev => ({
                    ...prev,
                    colors: [...prev.colors, selected]
                }))
            })
            .catch(e => {
                console.error(e)
                toast.danger("Failed to assign Color to Filament", {
                    description: `${e}`
                })
            })


    }

    return (
        <div className={"flex flex-col gap-4"}>
            <div className={"flex flex-col gap-2"}>
                {
                    props.filament.colors.map(c => (
                        <div className={"flex items-center gap-4"}>
                            <div
                                className={`flex items-center justify-center w-24 h-24 rounded-2xl clickable`}
                                style={{backgroundColor: `#${c.hex}`}}
                            />
                            <div>
                                <p>{c.name}</p>
                                <p>#{c.hex}</p>
                                <ButtonGroup size={"sm"} variant={"tertiary"}>
                                    <Button>
                                        <ChevronUpIcon/>
                                    </Button>
                                    <Button>
                                        <ChevronDownIcon/>
                                    </Button>
                                    <Button onClick={() => unassignColorClicked(c.id)}>
                                        <TrashIcon/>
                                    </Button>
                                </ButtonGroup>
                            </div>
                        </div>
                    ))
                }
                {
                    props.filament.colors.length === 0 && (
                        <Typography type={"body-sm"} color={"muted"}>This Filament has no colors</Typography>
                    )
                }
            </div>
            <TextField className={"flex gap-2"}>
                <Select
                    className={"w-full"}
                    onChange={value => onAssignChange(value)}
                >
                    <Label>Assign New Color...</Label>
                    <Select.Trigger>
                        <Select.Value/>
                        <Select.Indicator/>
                    </Select.Trigger>
                    <Select.Popover>
                        <ListBox>
                            {
                                assignableColors.map(c => (
                                    <ListBox.Item key={c.hex} id={c.hex} textValue={c.hex}>
                                        <div className={"flex items-center gap-2.5"}>
                                            <div className={"w-12 h-12 rounded-xl shadow"} style={{backgroundColor: `#${c.hex}`}} />
                                            <div className={"flex flex-col gap-0.5"}>
                                                <p>{c.name || ""}</p>
                                                <p>#{c.hex}</p>
                                            </div>
                                        </div>
                                    </ListBox.Item>
                                ))
                            }
                        </ListBox>
                    </Select.Popover>
                </Select>
            </TextField>
        </div>
    )
}
