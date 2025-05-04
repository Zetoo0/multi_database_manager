import { createFileRoute, Link, Outlet, useNavigate } from '@tanstack/react-router'
import { useLayout } from '../../providers/LayoutProvider'
import { useEffect } from 'react'
import { SplitLayout } from '../../components/layout/PageLayout/SplitLayout'
//import { ConnectionForm } from "../../components/forms/ConnectionForm";
import { useHotkey } from '../../hooks/hotkey/useHotkey'
import { ConnectionMenu } from '../../components/connection/ConnectionMenu'
import { Button } from '../../components/ui/Button'
import { X } from 'lucide-react'


export const Route = createFileRoute('/connect/_connection')({
  component: ConnectionPage
})

function ConnectionPage() {
  try {
    const navigate = useNavigate()
    const { setShowStatusBar, setShowTitleBar } = useLayout()
    const { formatted } = useHotkey('closeSettings', () => {
      navigate({ to: '/' })
    })
    useEffect(() => {
      setShowStatusBar(false)
      setShowTitleBar(false)
      return () => {
        setShowStatusBar(true)
        setShowTitleBar(true)
      }
    }, [])

    return (
      <SplitLayout
        left={<ConnectionMenu />}
        right={
            <div className='grid grid-cols-[auto_1fr] gap-2'>
                <Outlet />
                <Button
                variant="outline"
                className="sticky top size-8 rounded-full border-2 border-muted-foreground/70 p-0 text-muted-foreground/70 hover:border-muted-foreground"
                asChild
                >
                  <Link to="/">
                      <X strokeWidth="3" className="size-4" />
                      <span className="absolute inset-x top-full mx-auto mt-2 w-min text-xs uppercase">
                      {formatted}
                      </span>
                  </Link>
                </Button>
            </div>
        } 
      />
    )
  } catch (error) {
    console.error('Error in ConnectionPage:', error)
    return <div>Error loading ConnectionPage</div>
  }
}

/*

*/