import {
    SiActix, SiEspressif,
    SiGithub,
    SiGmail,
    SiInstagram,
    SiPostgresql,
    SiReact,
    SiRust, SiShadcnui, SiTailwindcss,
    SiTypescript
} from "@icons-pack/react-simple-icons";
import {ExternalLinkIcon} from "lucide-react";
import type {ReactElement} from "react";

export default function Footer() {
    return (
        <footer className={"flex items-center w-full h-52 gap-10 bg-neutral-100 border-t border-t-neutral-300 py-10"}>
            <div>
                <h4>Filamentizator</h4>
                <p className={"text-lg"}>© Peter Cyprich</p>
                <p>2026</p>
                <div className={"flex items-center gap-2 pt-2 mt-2 border-t border-t-neutral-300"}>
                    <a href={"https://github.com/cyprich/filamentizator"} className={"hoverable flex items-center gap-1.5"} target={"_blank"}>
                        Source Code
                        <ExternalLinkIcon className={"size-4"}/>
                    </a>
                </div>
            </div>
            <Separator/>
            <div className={"flex flex-col gap-3"}>
                <SocialLink icon={ <SiGithub/> } text={"cyprich"} link={"https://github.com/cyprich"} />
                <SocialLink icon={ <SiInstagram/> } text={"peter.tar.gz"} link={"https://www.instagram.com/peter.tar.gz"} />
                <SocialLink icon={ <SiGmail/> } text={"cypooriginal@gmail.com"} link={"mailto:cypooriginal@gmail.com"} />
            </div>
            <Separator/>
            <div className={"flex flex-col"}>
                <p className={"text-lg font-semibold"}>Tech stack</p>
                <div className={"flex gap-12"}>
                    <TechSection name={"Frontend"}>
                        <TechLink icon={ <SiReact/> } text={"React"} link={"https://react.dev/"} />
                        <TechLink icon={ <SiTypescript/> } text={"TypeScript"} link={"https://www.typescriptlang.org/"} />
                        <TechLink icon={ <SiTailwindcss/> } text={"TailwindCSS"} link={"https://tailwindcss.com"} />
                        <TechLink icon={ <SiShadcnui/> } text={"shadcn/ui"} link={"https://ui.shadcn.com/"} />
                    </TechSection>
                    <TechSection name={"Backend"}>
                        <TechLink icon={ <SiRust/> } text={"Rust"} link={"https://rust-lang.org/"} />
                        <TechLink icon={ <SiActix/> } text={"Actix-web"} link={"https://actix.rs/"} />
                    </TechSection>
                    <TechSection name={"Database"}>
                        <TechLink icon={ <SiPostgresql/> } text={"PostgreSQL"} link={"https://www.postgresql.org/"} />
                    </TechSection>
                    <TechSection name={"Device"}>
                        <TechLink icon={ <SiRust/> } text={"Rust no_std"} link={"https://github.com/esp-rs/esp-hal"} />
                        <TechLink icon={ <SiEspressif/> } text={"React"} link={"https://www.espressif.com/en/products/socs/esp32"} />
                    </TechSection>
                </div>
            </div>
        </footer>
    )
}

type LinkProps = {
    icon: any;
    text: string;
    link: string;
}

function SocialLink(props: LinkProps) {
    return (
        <div className={"flex items-center gap-2"}>
            {props.icon}
            <a href={props.link} target={"_blank"} className={"hoverable"}>{props.text}</a>
        </div>
    )
}

function TechLink(props: LinkProps) {
    return (
        <a className={"*:size-8 *:hover:scale-105"} href={props.link} target={"_blank"}>
            {props.icon}
        </a>
    )
}

type TechSectionProps = {
    name: string,
    children: ReactElement | ReactElement[]
}

function TechSection(props: TechSectionProps) {
    return (
        <div className={"flex flex-col gap-2"}>
            <p>{props.name}</p>
            <div className={"flex gap-2"}>
                {props.children}
            </div>
        </div>
    )
}

function Separator() {
    return (
        <div className={"h-full border-r border-r-neutral-300"} />
    )
}
