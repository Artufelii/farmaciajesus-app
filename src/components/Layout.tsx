import type { ReactElement } from "react"
import '../css/Layout.css'
import { Header } from "./Header"

type Props = {
  children: JSX.Element,
}

export function Layout({ children }: Props): ReactElement {
  return(
    <>
      <Header />
      <main className="layout">
        { children }
      </main>
    </>
  )
}
