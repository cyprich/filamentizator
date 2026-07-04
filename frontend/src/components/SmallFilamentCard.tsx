import {Card, Typography} from "@heroui/react";
import type {Filament} from "../types/filament.ts";
import {Link} from "react-router";
import {BoxIcon, CoinsIcon, FactoryIcon, ThermometerIcon, WeightIcon} from "lucide-react";

export type FilamentCardProps = {
    filament: Filament;
    className?: string;
}

export function SmallFilamentCard(props: FilamentCardProps) {
    return (
        <Link to={`/filaments/${props.filament.id}`}>
            <Card className={`w-64 bg-background **:text-foreground border ${props.className} clickable`}>
                <Card.Title>
                    <p className={"text-center font-semibold text-lg"}>
                        {props.filament.name}
                    </p>

                </Card.Title>
                <Card.Content className={"flex flex-col gap-4"}>
                    <img src={undefined} alt={"image"} className={"w-full aspect-square border"}/>
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
                            {props.filament.weight.net} of {props.filament.weight.original} g
                        </div>
                        <div>
                            <ThermometerIcon/>
                            <>
                                {props.filament.temp.min}
                                {
                                    props.filament.temp.max && ` - ${props.filament.temp.max}`
                                }
                                {" °C"}
                            </>
                        </div>
                        <div>
                            <CoinsIcon/>
                            {props.filament.price}
                            {' €'}
                        </div>
                    </div>
                </Card.Content>
            </Card>
        </Link>
    )
}


export function EmptySmallFilamentCard() {
    return (
        <Card className={`w-64 bg-background **:text-foreground border clickable`}>
            <Card.Content className={"flex items-center justify-center"}>
                <p className={"text-8xl font-extralight"}>+</p>
                <Typography type={"body"}>New Filament</Typography>
            </Card.Content>
        </Card>
    )
}
