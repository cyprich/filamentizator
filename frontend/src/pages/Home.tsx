import {type ReactElement, useEffect, useState} from "react";
import {fixVendorDates, type Vendor} from "@/models/vendor.ts";
import axios from "axios";
import {BASE_URL} from "@/main.tsx";
import {toast} from "sonner";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious} from "@/components/ui/carousel.tsx";
import {useNavigate} from "react-router";

export default function Home() {
    return (
        <main>
            <h1 className={"mb-12"}>Welcome to Filamentizator!</h1>
            <div className={"flex flex-col gap-16"}>
                <DummySection name={"Filaments"} redirect={"/"}><></></DummySection>
                <VendorsSection/>
                <DummySection name={"Materials"} redirect={"/"}><></></DummySection>
                <DummySection name={"Account"} redirect={"/"}><></></DummySection>
            </div>
        </main>
    )
}

type SectionProps = {
    name: string;
    redirect: string;
    children: ReactElement;
}

function DummySection(props: SectionProps) {
    const dummy = [1, 2, 3, 4];
    return (
        <div className={"flex flex-col gap-4"}>
            <h2>{props.name}</h2>
            <div className={"flex gap-8"}>
                {
                    dummy.map(() => (
                        <div className={"bg-neutral-200 w-48 h-48 flex items-center justify-center"}>
                            Placeholder
                        </div>
                    ))
                }
            </div>
        </div>
    )
}

function SectionWrapper(props: SectionProps) {
    const navigate = useNavigate();

    return (
        <div className={"flex flex-col gap-4"}>
            <h2 className={"-mb-4 hoverable"} onClick={() => navigate(props.redirect)}>{props.name}</h2>
            {props.children}
        </div>
    )
}

function VendorsSection() {
    const [vendors, setVendors] = useState<Vendor[]>([])

    useEffect(() => {
        axios.get<Vendor[]>(`${BASE_URL}/vendor`)
            .then(resp => {
                if (resp.data.length === 0) {
                    return
                }
                const newVendors = resp.data.map(v => (
                    fixVendorDates(v)
                ))
                setVendors(newVendors)
            })
            .catch(e => {
                toast.error(`Error while getting Vendors: ${e}`)
                console.error(e)
            })
    }, []);

    return (
        <SectionWrapper name={"Vendors"} redirect={"/vendors"}>
            <>
                <p>Total: {vendors.length}</p>
                {
                    vendors.length > 0
                        ? <Carousel className={"w-full"} opts={{
                            dragFree: true
                        }}>
                            <CarouselContent>
                                {
                                    vendors.map(v => (
                                        <CarouselItem className={"basis-1/4"}>
                                            <Card className={"select-none"}>
                                                <CardHeader>
                                                    <CardTitle><h3>{v.name}</h3></CardTitle>
                                                </CardHeader>
                                                <CardContent className={"flex flex-col gap-1"}>
                                                    <p>Last update: {v.date_edited}</p>
                                                    <p>Created: {v.date_created}</p>
                                                </CardContent>
                                            </Card>
                                        </CarouselItem>
                                    ))
                                }
                            </CarouselContent>
                            <CarouselPrevious/>
                            <CarouselNext/>
                        </Carousel>
                        : <p>bro add new one</p>
                }
                {

                }
            </>
        </SectionWrapper>
    )
}