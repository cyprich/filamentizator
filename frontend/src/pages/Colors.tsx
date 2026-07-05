import {
    Card, ColorArea,
    ColorField, ColorPicker, ColorSlider,
    InputGroup,
    Label, Popover,
    TextField,
    toast,
    Typography
} from "@heroui/react";
import {type Dispatch, type SetStateAction, useEffect, useState} from "react";
import type {Color} from "../types/color.ts";
import axios from "axios";
import {BASE_URL} from "../main.tsx";
import {CheckIcon, ChevronRightIcon, PencilIcon, SpoolIcon, TrashIcon, XIcon} from "lucide-react";
import type {Filament} from "../types/filament.ts";
import {Link} from "react-router";

export default function Colors() {
    const [colors, setColors] = useState<Color[]>([])
    const [filaments, setFilaments] = useState<Filament[]>([])

    useEffect(() => {
        axios.get<Color[]>(`${BASE_URL}/color`)
            .then(resp => setColors(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Failed to get Colors", {
                    description: `${e}`
                })
            })

        axios.get<Filament[]>(`${BASE_URL}/filament`)
            .then(resp => setFilaments(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Failed to get Filament-Color associations", {
                    description: `${e}`
                })
            })
    }, []);


    return (
        <main>
            <Typography type={"h2"}>Colors</Typography>
            <div className={"grid grid-cols-4 gap-4 mt-4"}>
                {
                    colors.map(c => (
                        <ColorCard
                            key={c.id}
                            color={c}
                            setColors={setColors}
                            filaments={filaments.filter(f => f.colors.map(c => c.id).includes(c.id))}
                        />
                    ))
                }
            </div>
            <div className={"my-8 border-b"}/>
            <div>
                <Typography type={"h4"}>Add new Color</Typography>
                //TODO
            </div>
        </main>
    )
}

type ColorCardProps = {
    color: Color;
    setColors: Dispatch<SetStateAction<Color[]>>;
    filaments: Filament[];
}

function ColorCard(props: ColorCardProps) {
    const [originalName, setOriginalName] = useState(props.color.name)
    const [originalHex, setOriginalHex] = useState(props.color.hex)

    const [name, setName] = useState<string | undefined>(props.color.name)
    const [hex, setHex] = useState<string>(props.color.hex)

    const [changes, setChanges] = useState<boolean>(false)

    // const [isExpanded, setIsExpanded] = useState<boolean>(false)

    // thanks chatgpt for this code
    const r = parseInt(hex.slice(0, 2), 16);
    const g = parseInt(hex.slice(2, 4), 16);
    const b = parseInt(hex.slice(4, 6), 16);

    // percieved brightness (yiq)
    const yiq = (r * 299 + g * 587 + b * 114) / 1000;
    const iconColor = yiq >= 128 ? "text-neutral-950" : "text-neutral-50"

    const confirmClicked = () =>  {
        axios.patch<Color>(`${BASE_URL}/color/${props.color.id}`, {
            name: name,
            hex: hex
        })
            .then(resp => {
                setOriginalName(resp.data.name)
                setOriginalHex(resp.data.hex)
                setName(resp.data.name)
                setHex(resp.data.hex)
            })
            .catch(e => {
                console.error(e)
                toast.danger("Color update failed", {
                    description: `${e}`
                })
            })
    }

    const cancelClicked = () =>  {
        setName(originalName)
        setHex(originalHex)
    }

    const deleteClicked = () => {
        axios.delete<Color>(`${BASE_URL}/color/${props.color.id}`)
            .then(resp => {
                props.setColors(prev => prev.filter(c => c.id !== resp.data.id))
            })
            .catch(e => {
                console.error(e)
                toast.danger("Failed to delete color", {
                    description: `${e}`
                })
            })
    }

    useEffect(() => {
        if (name !== originalName || hex !== originalHex) {
            setChanges(true)
        } else {
            setChanges(false)
        }
    }, [name, hex, originalName, originalHex]);

    return (
        <Card>
            <Card.Content className={"grid gap-4"} style={{gridTemplateColumns: "auto 1fr auto"}}>
                <ColorPicker
                    defaultValue={`#${hex}`}
                    onChange={val => setHex(val.toString("hex").slice(1))}
                >
                    <ColorPicker.Trigger>
                        <div
                            className={"flex justify-center items-center h-full w-22 rounded-xl clickable group"}
                            style={{backgroundColor: `#${hex}`}}
                        >
                            <PencilIcon className={`opacity-0 group-hover:opacity-100 ${iconColor}`}/>
                        </div>
                    </ColorPicker.Trigger>
                    <ColorPicker.Popover>
                        <ColorArea
                            colorSpace={"hsb"}
                            xChannel={"saturation"}
                            yChannel={"brightness"}
                        >
                            <ColorArea.Thumb/>
                        </ColorArea>
                        <ColorSlider channel={"hue"} colorSpace={"hsb"}>
                            <Label>Hue</Label>
                            <ColorSlider.Output/>
                            <ColorSlider.Track>
                                <ColorSlider.Thumb/>
                            </ColorSlider.Track>
                        </ColorSlider>
                    </ColorPicker.Popover>
                </ColorPicker>
                <div className={"flex flex-col gap-4"}>
                    <TextField>
                        <Label>Name</Label>
                        <InputGroup variant={"secondary"}>
                            <InputGroup.Input
                                value={name || ""}
                                onChange={e => setName(e.target.value)}
                                type={"string"}
                                placeholder={"Enter name..."}
                            />
                        </InputGroup>
                    </TextField>
                    <ColorField>
                        <Label>Color</Label>
                        <ColorField.Group variant={"secondary"}>
                            <ColorField.Prefix>#</ColorField.Prefix>
                            <ColorField.Input
                                value={hex}
                                onChange={e => setHex(e.target.value)}

                            />
                        </ColorField.Group>
                    </ColorField>
                </div>
                <div className={"flex flex-col gap-2 justify-center"}>
                    {
                        changes && ( <>
                            <CheckIcon
                                className={"clickable-no-scale hover:text-success"}
                                onClick={confirmClicked}
                            />
                            <XIcon
                                className={"clickable-no-scale hover:text-danger"}
                                onClick={cancelClicked}
                            />
                        </> )
                    }
                    <Popover>
                        <Popover.Trigger>
                            <SpoolIcon className={"clickable-no-scale hover:text-blue-600"}/>
                        </Popover.Trigger>
                        <Popover.Content placement={"left"}>
                            <Popover.Dialog>
                                <Popover.Arrow/>
                                <Popover.Heading className={"mb-2"}>
                                    <Typography type={"body"} className={"-mb-1.5"}>Associated Filaments</Typography>
                                    <Typography type={"body-sm"} color={"muted"}>Which Filaments has this color</Typography>
                                </Popover.Heading>
                                <div className={"flex flex-col gap-4"}>
                                    {
                                        props.filaments.length > 0
                                            ? props.filaments.map(f => (
                                                <Link to={`/filaments/${f.id}`}>
                                                    <Card variant={"secondary"} className={"rounded-xl"}>
                                                        <Card.Content className={"flex flex-row! justify-between items-center "}>
                                                            <div>
                                                                <p className={"font-semibold"}>{f.name}</p>
                                                                <p>{f.vendor.name} | {f.material.name}</p>
                                                            </div>
                                                            <ChevronRightIcon/>
                                                        </Card.Content>
                                                    </Card>
                                                </Link>
                                            ))
                                            : <Typography color={"muted"} type={"body-sm"}>No associated filaments</Typography>
                                    }
                                </div>
                            </Popover.Dialog>
                        </Popover.Content>
                    </Popover>
                    <TrashIcon className={"clickable-no-scale hover:text-danger"} onClick={deleteClicked}/>
                </div>
            </Card.Content>
        </Card>
    )
}