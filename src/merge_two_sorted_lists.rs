use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option< Box<ListNode> >
}

impl ListNode
{
    #[inline]
    pub fn new(val: i32) -> Self
    {
        return ListNode{ val, next: None };
    }

    #[inline]
    pub fn with_next(val: i32, next: ListNode) -> Self
    {
        return ListNode{ val, next: Some(Box::new(next)) };
    }

    pub fn from_vec(vals: &Vec<i32>) -> Self
    {
        let mut root_node: ListNode = ListNode::new(vals[0]);

        let mut curr_node: &mut Option< Box<ListNode> > = &mut root_node.next;

        for i in 1..vals.len()
        {
            *curr_node = Some(Box::new(ListNode::new(vals[i])));

            if let Some(next) = curr_node // next is &mut Box<ListNode>
            {
                curr_node = &mut (*next).deref_mut().next;
            }
            else
            {
                break;
            }
        }

        return root_node;
    }

    pub fn to_vec(&self) -> Vec<i32>
    {
        let mut vals: Vec<i32> = vec![self.val];

        let mut next_opt = &self.next;

        while let Some(next) = next_opt
        {
            vals.push(next.deref().val);

            next_opt = &next.deref().next;
        }

        return vals;
    }

    pub fn to_string(&self) -> String
    {
        let mut result_string: String = self.val.to_string();

        let mut next_opt: &Option< Box<ListNode> > = &self.next;

        while let Some(next) = next_opt.as_deref()
        {
            result_string.push_str(format!(" -> {}", next.val).as_str());

            next_opt = &next.next;
        }

        result_string.push_str(" -> None");

        return result_string;
    }
}

pub fn merge_two_lists(list1: Option< Box<ListNode> >, list2: Option< Box<ListNode> >) -> Option< Box<ListNode> >
{
    return match (list1, list2)
    {
        (None, None) => None,
        (None, Some(l2)) => Some(l2),
        (Some(l1), None) => Some(l1),
        (Some(l1), Some(l2)) =>
        {
            let vec1 = (*l2).to_vec();
            let vec2 = (*l1).to_vec();

            println!("vec1: {:#?}", vec1);
            println!("vec2: {:#?}", vec2);

            let mut merged_vec: Vec<i32> = Vec::new();

            let mut v1_index: usize = 0;
            let mut v2_index: usize = 0;

            while v1_index != vec1.len() && v2_index != vec2.len()
            {
                println!("{:#?}\nv1_index: {}\nv2_index: {}\n", merged_vec, v1_index, v2_index);

                if vec1[v1_index] < vec2[v2_index]
                {
                    merged_vec.push(vec1[v1_index]);

                    v1_index += 1;
                }
                else
                {
                    merged_vec.push(vec2[v2_index]);

                    v2_index += 1;
                }
            }

            while v1_index != vec1.len()
            {
                println!("{:#?}\nv1_index: {}\nv2_index: {}\n", merged_vec, v1_index, v2_index);

                merged_vec.push(vec1[v1_index]);

                v1_index += 1;
            }

            while v2_index != vec2.len()
            {
                println!("{:#?}\nv1_index: {}\nv2_index: {}\n", merged_vec, v1_index, v2_index);

                merged_vec.push(vec2[v2_index]);

                v2_index += 1;
            }

            Some(Box::new(ListNode::from_vec(&merged_vec)))
        }
    }
}

#[test]
pub fn test()
{
    let list1 = ListNode::with_next(1, ListNode::new(5));
    let list2 = ListNode::new(3);

    let res = ListNode::with_next(1, ListNode::with_next(3, ListNode::new(5)));

    let eres = merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2))).unwrap();
    let eres = eres.deref().to_owned();

    assert_eq!(res, eres);
}
