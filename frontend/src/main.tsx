import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import './styles/classes.css'
import './styles/elements.css'
import {BrowserRouter, Route, Routes} from "react-router";
import Filaments from "@/pages/Filaments.tsx";
import CustomSidebar from "@/components/CustomSidebar.tsx";
import Home from "@/pages/Home.tsx";
import Vendors from "@/pages/Vendors.tsx";
import Materials from "@/pages/Materials.tsx";
import Account from "@/pages/Account.tsx";
import Footer from "@/components/Footer.tsx";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <CustomSidebar/>
      <div className={"flex flex-col"}>
        <Routes>
          <Route path="/" element={<Home/>}/>
          <Route path="/filaments" element={<Filaments/>}/>
          <Route path="/vendors" element={<Vendors/>}/>
          <Route path="/materials" element={<Materials/>}/>
          <Route path="/account" element={<Account/>}/>
        </Routes>
        <Footer/>
      </div>
    </BrowserRouter>
  </StrictMode>,
)
