use zoon::*;
use ulid::Ulid;
use std::borrow::Cow;
use chrono::{prelude::*, Duration};
use crate::app;
use shared::{ClientId, TimeBlockId, InvoiceId, TimeBlockStatus};

pub mod els;

blocks!{
    append_blocks![els]

    #[subscription]
    fn on_route_change() {
        if let app::Route::ClientsAndProjects = route() {
            added_time_block().set(None);
            app::send_up_msg(false, UpMsg::GetTimeBlocksClients);
        }
    }

    #[subscription]
    fn handle_down_msg() {
        app::down_msg().inner().try_update(|down_msg| {
            match down_msg {
                Some(DownMsg::TimeBlocksClients(clients)) => {
                    set_clients(clients);
                    None
                }
                _ => down_msg
            }
        });
    }

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        time_blocks: Vec<Var<TimeBlock>>,
        tracked: Duration,
    }

    #[var]
    fn clients() -> Option<Vec<Var<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::time_blocks::Client>) {
        let clients = clients.into_iter().map(|client| {

            let client_var = Var::new(Client {
                id: client.id,
                name: client.name,
                projects: Vec::new(),
            });

            let projects = client.projects.into_iter().map(|project| {
                let project_var = Var::new(Project {
                    id: project.id,
                    name: project.name,
                    client: client_var,
                });
            }).collect();

            client_var.update_mut(move |client| {
                client.projects = projects;
            });
            client_var

        }).collect();
        clients().set(Some(clients));
    }

    // ------ TimeBlock ------

    #[derive(Debug)]
    struct TimeBlocks {
        id: TimeBLockId,
        name: String,
        status: TimeBlockStatus,
        duration: Duration,
        invoice: Option<Var<Invoice>>,
        client: Var<Client>, 
    }

    #[var]
    fn added_time_block() -> Option<Var<TimeBlock>> {
        None
    }

    #[update]
    fn add_time_block(client: Var<Client>) {
        let previous_duration = client.try_map(|client| {
            client.time_blocks
                .iter()
                .next_back()
                .map(|time_block| time_block.duration)
        }).flatten();

        let time_block = TimeBlock {
            id: TimeBlockId::new(),
            name: String::new(),
            status: TimeBlockStatus::default(),
            duration: previous_duration.unwrap_or_else(|| Duration::hours(20)),
            invoice: None,
            client,
        };
        client.try_update_mut(move |client| {
            app::send_up_msg(
                true, 
                UpMsg::AddTimeBlock(client.id, time_block.id, time_block.duration)
            );
            let time_block = Var::new(time_block);
            added_time_block().set(Some(time_block));
            client.time_blocks.push(time_block);
        });
    }

    #[update]
    fn remove_time_block(time_block: Var<TimeBlock>) {
        if let Some(removed_time_block) = time_block.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveTimeBlock(removed_time_block.id));
            removed_time_block.client.try_update_mut(|client| {
                if let Some(position) = client.time_blocks.iter().position(|tb| tb == time_block) {
                    clients.time_blocks.remove(position);
                }
            });
        }
    }

    #[update]
    fn rename_time_block(time_block: Var<TimeBlock>, name: &str) {
        time_block.try_use_ref(|time_block| {
            app::send_up_msg(true, UpMsg::RenameTimeBlock(time_block.id, Cow::from(name)));
        });
    }

    #[update]
    fn set_time_block_status(time_block: Var<TimeBlock>, status: TimeBlockStatus) {
        time_block.try_update_mut(|time_block| {
            time_block.status = status;
            app::send_up_msg(true, UpMsg::SetTimeBlockStatus(time_block.id, status));
        });
    }

    #[update]
    fn set_time_block_duration(time_block: Var<TimeBlock>, duration: Duration) {
        time_block.try_update_mut(|time_block| {
            time_block.duration = duration;
            app::send_up_msg(true, UpMsg::SetTimeBlockDuration(time_block.id, duration));
        });
    }

    // ------ Invoice ------

    #[derive(Debug)]
    struct Invoice {
        id: InvoiceId,
        custom_id: String,
        url: String, 
        time_block: Var<TimeBlock>, 
    }

    #[update]
    fn add_invoice(time_block: Var<TimeBlock>) {
        let invoice = Invoice {
            id: InvoiceId::new(),
            custom_id: String::new(),
            url: String::new(),
            time_block,
        };
        time_block.try_update_mut(move |time_block| {
            app::send_up_msg(true, UpMsg::AddInvoice(time_block.id, invoice.id));
            time_block.invoice = Some(Var::new(invoice));
        });
    }

    #[update]
    fn remove_invoice(invoice: Var<Invoice>) {
        if let Some(removed_invoice) = invoice.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveInvoice(removed_invoice.id));
            removed_invoice.time_block.try_update_mut(|time_block| {
                time_block.invoice = None;
            });
        }
    }

    #[update]
    fn set_invoice_custom_id(invoice: Var<Invoice>, custom_id: &str) {
        invoice.try_use_ref(|invoice| {
            app::send_up_msg(true, UpMsg::SetInvoiceCustomId(invoice.id, Cow::from(custom_id)));
        });
    }

    #[update]
    fn set_invoice_url(invoice: Var<Invoice>, url: &str) {
        invoice.try_use_ref(|invoice| {
            app::send_up_msg(true, UpMsg::SetInvoiceUrl(invoice.id, Cow::from(url)));
        });
    }

}
