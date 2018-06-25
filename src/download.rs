// use reqwest
// Client?

fn download(target: &str, quiet_mode: bool) -> Result<(), Box<::std::error::Error> {
    // parse URL
    let url = parse_url(target)?;
    let client = Client::new().unwrap(); // what's unwrap
    let mut resp = client.get(url)?
        .send()
        .unwrap();
    print(format!("HTTP request was sent... {}",
                  style(format!("{}", resp.status())).green()),
          quiet_mode); // wouldn't true be flipped here?
    if resp.status().is_success() {
        let headers = resp.headers().clone(); // why clone? perhaps instead of ref?
        // What is this part?
        let ct_len = headers.get::<ContentLength>().map(|ct_len| **ct_len);
        let ct_type = headers.get::<ContentType>().unwrap();

        match ct_len {
            Some (len) => { // new variable declared?
                print(format!("Length: {} ({})",
                      style(len).green(),
                      style(format!("{}", HumanBytes(len))).red()),
                    quiet_mode);
            },
            None => {
                print(format!("Length: {}", style("unknown"),red()), quiet_mode);
            },
        }

        print(format!("Type: {}", style(ct_type).green()), quite_mode);

        let fname = target.split("/").last().unwrap();

        print(format!("Saving to: {}", style(fname).green()), quiet_mode);

        let chunk_size = match ct_len {
            Some(x) => x as usize / 99, // what is usize?
            None => 1024usize, // ?
        };

        let mut buf = Vec::new(); // what is vec/buf?

        let bar = create_progress_bar(quite_mode, fname, ct_len); // do i need to import at top?

        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..]).unwrap();
            buffer.truncate(bcount); // ?
            if !buffer.is_empty() {
                buf.extend(buffer.into_boxed_slice()
                                 .into_vec()
                                 .iter()
                                 .cloned());
                bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        bar.finish();

        save_to_file(&mut buf, fname)?; // where is this coming from?

    }

    Ok(()) // what's this?

}

