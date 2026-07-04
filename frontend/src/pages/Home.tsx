import {Card, Typography} from "@heroui/react";
import {type ReactNode, useEffect, useState} from "react";
import type {Filament} from "../types/filament.ts";
import axios from "axios";
import {BASE_URL} from "../main.tsx";
import {BoxIcon, ChevronRight, CoinsIcon, FactoryIcon, ThermometerIcon, WeightIcon} from "lucide-react";
import type {Vendor} from "../types/vendor.ts";
import type {Material} from "../types/material.ts";
import {NameTable} from "../components/NameTable.tsx";
import {Link} from "react-router";

export default function Home() {
    const [filaments, setFilaments] = useState<Filament[]>([])
    const [vendors, setVendors] = useState<Vendor[]>([])
    const [materials, setMaterials] = useState<Material[]>([])

    // TODO toasts
    useEffect(() => {
        axios.get<Filament[]>(`${BASE_URL}/filament`)
            .then(resp => setFilaments(resp.data))
            .catch(e => console.error(e))
        axios.get<Vendor[]>(`${BASE_URL}/vendor`)
            .then(resp => setVendors(resp.data))
            .catch(e => console.error(e))
        axios.get<Material[]>(`${BASE_URL}/material`)
            .then(resp => setMaterials(resp.data))
            .catch(e => console.error(e))

    }, []);

    return (
        <main className={"flex flex-col gap-16"}>
            <Typography color={"muted"} className={"-mb-14"}>Welcome to filamentizator!</Typography>
            <Section
                title={"Filaments"}
                titleLink={"/filaments"}
                description={`Total: ${filaments.length} filament${filaments.length > 1 ? 's' : ''}`}
                className={"flex gap-6"}
            >
                {
                    filaments.map(f => (
                        <FilamentCard filament={f}/>
                    ))
                }
                <FilamentCard/>
            </Section>
            <div className={"grid grid-cols-2 gap-8"}>
                <Section title={"Vendors"}>
                    <NameTable data={vendors} dataName={"Vendor"} paginationSize={10}/>
                </Section>
                <Section title={"Materials"}>
                    <NameTable data={materials} dataName={"Material"} paginationSize={10}/>
                </Section>
            </div>
            <Section title={"Account"} titleLink={"/account"}>//TODO</Section>
        </main>
    )
}

type SectionProps = {
    title: string;
    titleLink?: string;
    description?: string;
    children: ReactNode;
    className?: string;
}

function Section(props: SectionProps) {
    return (
        <div>
            {
                props.titleLink
                    ? <Link to={props.titleLink} className={"flex items-center gap-1 group clickable hover:scale-100! w-max"}>
                        <Typography type={"h2"}>{props.title}</Typography>
                        <ChevronRight className={"opacity-10 group-hover:opacity-100 size-8"}/>

                    </Link>
                    : <Typography type={"h2"}>{props.title}</Typography>

            }
            {
                props.description && <Typography type={"body-sm"} className={"mb-2"}>
                    { props.description }
                </Typography>
            }
            <div className={`pt-4 ${props.className}`}>
                { props.children }
            </div>
        </div>
    )
}

type FilamentCardProps = {
    filament?: Filament;
    className?: string;
}

function FilamentCard(props: FilamentCardProps) {
    return (
        <Card className={`w-64 bg-background **:text-foreground border ${props.className} clickable`}>
            {
                props.filament
                    ? <>
                        <Card.Title>
                            <p className={"text-center font-semibold text-lg"}>
                                {props.filament.name}
                            </p>

                        </Card.Title>
                        <Card.Content className={"flex flex-col gap-4"}>
                            <img src={undefined} alt={"image"} className={"w-full aspect-square border"} />
                            <div className={"flex flex-col gap-2 *:flex *:gap-2"}>
                                <div>
                                    <FactoryIcon/>
                                    {props.filament.vendor.name}
                                </div>
                                <div>
                                    <BoxIcon/>
                                    {props.filament.material.name}
                                </div>
                                <div>
                                    <WeightIcon/>
                                    zatial nic
                                </div>
                                <div>
                                    <ThermometerIcon/>
                                    <>
                                        {props.filament.temp.min}
                                        {
                                            props.filament.temp.max && ` - ${props.filament.temp.max}`
                                        }
                                        { " °C"}
                                    </>
                                </div>
                                <div>
                                    <CoinsIcon/>
                                    { props.filament.price }
                                    { ' €' }
                                </div>
                            </div>
                        </Card.Content>
                    </>
                    : <Card.Content className={"flex items-center justify-center"}>
                        <p className={"text-8xl font-extralight"}>+</p>
                        <Typography type={"body"}>New Filament</Typography>
                    </Card.Content>
            }
        </Card>
    )
}
