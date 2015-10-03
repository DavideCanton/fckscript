use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, LineWriter, BufRead, Write};

fn indent(indent: u32) -> String
{
    let mut s = String::new();
    for _ in 0..indent
    {
        s.push_str("\t");
    }
    return s;
}

fn append_transl(c: char, v: &mut Vec<String>, cur_indent: &mut u32)
{
    match c
    {
        '+' => v.push(indent(*cur_indent) + "memory[mp] = (memory[mp] + 1) & 0xFF;"),
        '-' => v.push(indent(*cur_indent) + "memory[mp] = (memory[mp] - 1) & 0xFF;"),
        '>' => v.push(indent(*cur_indent) + "mp++; if(mp > L) mp = 0;"),
        '<' => v.push(indent(*cur_indent) + "mp--; if(mp < 0) mp = L - 1;"),
        ',' => v.push(indent(*cur_indent) + "memory[mp] = bf_read().charCodeAt(0);"),
        '.' => v.push(indent(*cur_indent) + "bf_write(String.fromCharCode(memory[mp]));"),
        '[' => {
            v.push(indent(*cur_indent) + "while(memory[mp]) {");
            *cur_indent += 1;
        },
        ']' => {
            *cur_indent -= 1;
            v.push(indent(*cur_indent) + "}");
        },
        _   => ()
    }
}

fn convert_line(line: String, cur_indent: &mut u32) -> Vec<String>
{
    let mut v = vec![];
    for c in line.chars()
    {
        append_transl(c, &mut v, cur_indent);
    }
    return v;
}

fn write_header(out_bw: &mut Write)
{
    writeln!(out_bw, "function execute_program() {{").unwrap();
    writeln!(out_bw, "\tif(!bf_read) console.error(\"bf_read function not found!\");").unwrap();
    writeln!(out_bw, "\tif(!bf_write) console.error(\"bf_write function not found!\");").unwrap();
    writeln!(out_bw, "\tvar memory = [];").unwrap();
    writeln!(out_bw, "\tvar L = 30000;").unwrap();
    writeln!(out_bw, "\tfor(_i = 0; _i < L; _i++) memory.push(0);").unwrap();
    writeln!(out_bw, "\tvar mp = 0;").unwrap();
    writeln!(out_bw, "").unwrap();
}

fn write_footer(out_bw: &mut Write)
{
    writeln!(out_bw, "").unwrap();
    writeln!(out_bw, "}}").unwrap();
}

pub fn main()
{
    let arg = env::args().nth(1).expect("Missing file name!");
    let path_src = Path::new(&arg);
    let path_dst = path_src.with_extension("js");

    {
        let f_in = File::open(&path_src)
            .ok()
            .expect(&format!("Error in opening {}!", path_src.to_str().unwrap()));

        let f_out = File::create(&path_dst)
            .ok()
            .expect(&format!("Error in opening {}!", path_dst.to_str().unwrap()));

        let in_br = BufReader::new(f_in);
        let mut out_bw = LineWriter::new(f_out);

        write_header(&mut out_bw);
        let mut cur_indent = 1;

        for line in in_br.lines()
        {
            let res = convert_line(line.unwrap(), &mut cur_indent);
            for res_line in res
            {
                if let Err(_) = write!(out_bw, "{}\n", res_line)
                {
                    panic!("Error in writing!");
                }
            }
        }

        write_footer(&mut out_bw);
    }
}
